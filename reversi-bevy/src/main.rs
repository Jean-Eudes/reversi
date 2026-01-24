use crate::GameState::{EndGame, InGame, Start};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use reversi_core::application::use_case::UseCase;
use reversi_core::domain::board::ColorPiece::Black;
use reversi_core::domain::board::{Board, BoardIter, Case, ColorPiece};
use ColorPiece::White;
use TurnState::{AiThinking, HumanTurn};

const CELL_SIZE: f32 = 60f32;

#[derive(Resource)]
struct BoardResource(Board);

#[derive(Resource)]
struct UseCaseResource(UseCase);

#[derive(Resource)]
struct GameAssets {
    pawn_atlas_layout: Handle<TextureAtlasLayout>,
    pawn_texture: Handle<Image>,
    texture_wood: Handle<Image>,
}

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
}

fn main() {
    let use_case = UseCase::default();
    let board = use_case.initialize_game_use_case.execute();

    App::new()
        //.insert_resource(WinitSettings::desktop_app())
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
        .add_systems(OnEnter(EndGame), display_end_game)
        .add_systems(
            Update,
            (
                create_board.run_if(in_state(Start)),
                check_end_game,
                handle_click
                    .run_if(in_state(InGame(HumanTurn)).and(input_just_pressed(MouseButton::Left))),
                ai_play_system.run_if(in_state(InGame(AiThinking))),
                execute_player_move.run_if(on_message::<MoveAccepted>),
                apply_move.run_if(on_message::<MoveProcessed>),
            )
                .chain(),
        )
        .run();
}

fn init_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let texture_pions = asset_server.load("pions.png");
    let texture_wood = asset_server.load("woodTexture.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 2, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets {
        pawn_atlas_layout: layout_handle,
        pawn_texture: texture_pions,
        texture_wood,
    });

}
fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_res: ResMut<BoardResource>,
    mut next_state: ResMut<NextState<GameState>>,
    assets: Res<GameAssets>,
) {
    let vertical_segment = meshes.add(Segment2d::new(
        Vec2::new(0f32, -CELL_SIZE * 4f32),
        Vec2::new(0f32, CELL_SIZE * 4f32),
    ));
    let horizontal_segment = meshes.add(Segment2d::new(
        Vec2::new(-CELL_SIZE * 4f32, 0f32),
        Vec2::new(CELL_SIZE * 4f32, 0f32),
    ));

    commands.spawn((
        InitGame,
        Sprite {
            image: assets.texture_wood.clone(),
            //custom_size: Some(Vec2::new(64.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    let rectangle = meshes.add(Rectangle::new(CELL_SIZE * 8f32, CELL_SIZE * 8f32));

    let black = Color::linear_rgb(0., 0., 0.);
    let green_reversi = Color::linear_rgb(0.0, 0.4, 0.0);

    let black_color_handle = materials.add(black);
    let green_reversi_color_handle = materials.add(green_reversi);

    commands.spawn((
        InitGame,
        Mesh2d(rectangle),
        MeshMaterial2d(green_reversi_color_handle.clone()),
        Transform::from_xyz(CELL_SIZE * -0f32, CELL_SIZE * 0f32, 0f32),
    ));

    for i in -4..=4 {
        commands.spawn((
            InitGame,
            Mesh2d(vertical_segment.clone()),
            MeshMaterial2d(black_color_handle.clone()),
            Transform::from_xyz(CELL_SIZE * i as f32, 0f32, 1f32),
        ));

        commands.spawn((
            InitGame,
            Mesh2d(horizontal_segment.clone()),
            MeshMaterial2d(black_color_handle.clone()),
            Transform::from_xyz(0f32, CELL_SIZE * i as f32, 1f32),
        ));
    }
    for (x, y) in BoardIter::default() {
        if let Some(Case::Piece(color)) = game_res.0.cell(x, y) {
            add_piece(&mut commands, x, y, color, &assets);
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
    mut query: Query<(&CaseUi, &mut Sprite)>,
    mut message_reader: MessageReader<MoveProcessed>,
    assets: Res<GameAssets>,
) {
    for move_processed in message_reader.read() {
        for (case, mut sprite) in &mut query {
            if move_processed.pieces_to_flip.contains(&(case.x, case.y))
                && let Some(sprite) = sprite.texture_atlas.as_mut()
            {
                sprite.index = if move_processed.player == Black { 1 } else { 0 };
            }
        }

        add_piece(
            &mut commands,
            move_processed.position.0,
            move_processed.position.1,
            &move_processed.player,
            &assets,
        );
    }
}

fn add_piece(
    commands: &mut Commands,
    x: usize,
    y: usize,
    color: &ColorPiece,
    assets: &Res<GameAssets>,
) {
    commands.spawn((
        CaseUi { x, y },
        Sprite {
            image: assets.pawn_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.pawn_atlas_layout.clone(),
                index: if color == &White { 0 } else { 1 },
            }),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(
            (x as isize - 4) as f32 * CELL_SIZE + CELL_SIZE / 2.,
            (-(y as isize) + 4) as f32 * CELL_SIZE - CELL_SIZE / 2.,
            1f32,
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
