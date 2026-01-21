use crate::GameState::{EndGame, EndScreen, InGame, Start};
use ColorPiece::White;
use TurnState::{AiThinking, HumanTurn};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy::winit::WinitSettings;
use reversi_core::application::use_case::UseCase;
use reversi_core::domain::board::ColorPiece::Black;
use reversi_core::domain::board::{Board, BoardIter, Case, ColorPiece};

const CELL_SIZE: f32 = 60f32;

#[derive(Resource)]
struct BoardResource(Board);

#[derive(Resource)]
struct UseCaseResource(UseCase);

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

#[derive(Component)]
struct CaseUi {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct InitGame;

#[derive(Message)]
struct MoveAccepted {
    x: usize,
    y: usize,
}
#[derive(Message)]
struct MoveProcessed {
    position: (usize, usize),
    pieces_to_flip: Vec<(usize, usize)>,
    player: ColorPiece,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum TurnState {
    #[default]
    HumanTurn,
    AiThinking,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Start,
    InGame(TurnState),
    EndGame,
    EndScreen
}

fn main() {
    let use_case = UseCase::default();
    let board = use_case.initialize_game_use_case.execute();

    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(BoardResource(board))
        .insert_resource(UseCaseResource(use_case))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Reversi - Bevy Edition".into(),
                resolution: WindowResolution::new(560, 560),
                // Empêche le redimensionnement si tu veux garder ta grille propre
                resizable: false,
                // Optionnel : synchronisation verticale (Vsync)
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_message::<MoveAccepted>()
        .add_message::<MoveProcessed>()
        .add_systems(Startup, init_game)
        .add_systems(Update, tick_despawn_timers)
        .add_systems(
            Update,
            (
                create_board.run_if(in_state(Start)),
                check_end_game,
                handle_click
                    .run_if(in_state(InGame(HumanTurn)).and(input_just_pressed(MouseButton::Left))),
                ai_play_system.run_if(in_state(InGame(AiThinking))),
                display_end_game.run_if(in_state(EndGame)),
                execute_player_move.run_if(on_message::<MoveAccepted>),
                apply_move.run_if(on_message::<MoveProcessed>),
            )
                .chain(),
        )
        .run();
}

fn init_game(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_res: ResMut<BoardResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let vertical_segment = meshes.add(Segment2d::new(
        Vec2::new(0f32, -CELL_SIZE * 4f32),
        Vec2::new(0f32, CELL_SIZE * 4f32),
    ));
    let horizontal_segment = meshes.add(Segment2d::new(
        Vec2::new(-CELL_SIZE * 4f32, 0f32),
        Vec2::new(CELL_SIZE * 4f32, 0f32),
    ));

    let color = Color::linear_rgb(0., 0., 0.);

    for i in -4..=4 {
        let handle = materials.add(color);
        commands.spawn((
            InitGame,
            Mesh2d(vertical_segment.clone()),
            MeshMaterial2d(handle.clone()),
            Transform::from_xyz(CELL_SIZE * i as f32, 0f32, 0f32),
        ));

        commands.spawn((
            InitGame,
            Mesh2d(horizontal_segment.clone()),
            MeshMaterial2d(handle),
            Transform::from_xyz(0f32, CELL_SIZE * i as f32, 0f32),
        ));
    }
    for (x, y) in BoardIter::default() {
        if let Some(Case::Piece(color)) = game_res.0.cell(x, y) {
            add_piece(&mut commands, &mut meshes, &mut materials, x, y, color);
        }
    }

    next_state.set(InGame(HumanTurn));
}

fn handle_click(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut message_writer: MessageWriter<MoveAccepted>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();

    // 1. Récupérer la position du curseur en pixels
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        // 'world_position' est maintenant un Vec2 dans ton repère Bevy (0,0 au centre)
        if mouse_input.just_pressed(MouseButton::Left) {
            let x = ((world_position.x + CELL_SIZE * 4f32) / CELL_SIZE) as usize;
            let y = ((CELL_SIZE * 4. - world_position.y) / CELL_SIZE) as usize;
            message_writer.write(MoveAccepted { x, y });
        }
    }
}

fn ai_play_system(
    mut game: ResMut<BoardResource>,
    use_case: ResMut<UseCaseResource>,
    mut next_state: ResMut<NextState<GameState>>,
    mut message_writer: MessageWriter<MoveProcessed>,
) {
    let board = &mut game.0;
    let use_case = &use_case.0;
    let move_ia = use_case.play_ai_move_use_case.execute(board);

    if let Some(selected_move) = move_ia {
        message_writer.write(MoveProcessed {
            position: selected_move.position(),
            pieces_to_flip: selected_move.pieces_to_flip(),
            player: White,
        });
        if board.player1() {
            next_state.set(InGame(HumanTurn));
        }
    }
}

fn execute_player_move(
    mut game_res: ResMut<BoardResource>,
    use_case: ResMut<UseCaseResource>,
    mut message_reader: MessageReader<MoveAccepted>,
    mut message_writer: MessageWriter<MoveProcessed>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for move_accepted in message_reader.read() {
        let board = &mut game_res.0;
        let option = use_case
            .0
            .play_move_use_case
            .execute(board, move_accepted.x, move_accepted.y);
        if let Some(flip_pieces) = option {
            message_writer.write(MoveProcessed {
                position: (move_accepted.x, move_accepted.y),
                pieces_to_flip: flip_pieces,
                player: Black,
            });
            if board.player2() {
                next_state.set(InGame(AiThinking));
            }
        }
    }
}

fn apply_move(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&CaseUi, &mut MeshMaterial2d<ColorMaterial>)>,
    mut message_reader: MessageReader<MoveProcessed>,
) {
    for move_processed in message_reader.read() {
        for content in &query {
            if move_processed
                .pieces_to_flip
                .contains(&(content.0.x, content.0.y))
                && let Some(mat) = materials.get_mut(&content.1.0)
            {
                mat.color = if move_processed.player == Black {
                    Color::BLACK
                } else {
                    Color::WHITE
                };
            }
        }

        add_piece(
            &mut commands,
            &mut meshes,
            &mut materials,
            move_processed.position.0,
            move_processed.position.1,
            &move_processed.player,
        );
    }
}

fn add_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: usize,
    y: usize,
    color: &ColorPiece,
) {
    let color_black = Color::linear_rgb(0., 0., 0.);
    let color_white = Color::linear_rgb(1., 1., 1.);

    let circle = Circle::new(20f32);

    let handle = meshes.add(circle);

    commands.spawn((
        CaseUi { x, y },
        Mesh2d(handle),
        MeshMaterial2d(materials.add(if color == &Black {
            color_black
        } else {
            color_white
        })),
        Transform::from_xyz(
            (x as isize - 4) as f32 * CELL_SIZE + CELL_SIZE / 2.,
            (-(y as isize) + 4) as f32 * CELL_SIZE - CELL_SIZE / 2.,
            0f32,
        ),
    ));
}

fn check_end_game(
    game_res: ResMut<BoardResource>,
    use_case: ResMut<UseCaseResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if use_case
        .0
        .evaluate_game_end_use_case
        .execute(&game_res.0)
        .is_some()
    {
        println!("End game");
        next_state.set(EndGame);
    }
}
fn display_end_game(
    mut game_res: ResMut<BoardResource>,
    query: Query<Entity, Or<(With<CaseUi>, With<InitGame>)>>,
    use_case: ResMut<UseCaseResource>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    println!("remove all element");
    for input in query {
        commands.entity(input).despawn();
    }

    let text = Text2d::new("Victoire du joueur");

    commands.spawn((
        text,
        Transform::from_xyz(0., 0., 10.),
        DespawnTimer(Timer::from_seconds(2.0, TimerMode::Once)),
    ));
    let board = use_case.0.initialize_game_use_case.execute();
    game_res.0 = board;
    next_state.set(EndScreen);
}

fn tick_despawn_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut timer) in &mut query {
        // On fait avancer le timer avec le temps écoulé depuis la dernière frame
        timer.0.tick(time.delta());

        // Si le timer est terminé, on supprime l'entité
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
            next_state.set(Start);
        }

    }
}