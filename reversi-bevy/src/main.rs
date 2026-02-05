mod fireworks;
mod menu;

use crate::GameState::{EndGame, GameOverScreen, InGame, Menu};
use crate::fireworks::{Firework, FireworkPlugin};
use crate::menu::MenuPlugin;
use ColorPiece::White;
use TurnState::{AiThinking, AiWaiting, HumanTurn};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use reversi_core::application::use_case::UseCase;
use reversi_core::domain::board::ColorPiece::Black;
use reversi_core::domain::board::{Board, BoardIter, Case, ColorPiece};

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
struct PlayableIndicator;

#[derive(Component)]
struct BoardRoot;

#[derive(Event)]
struct MoveAccepted {
    x: usize,
    y: usize,
}

#[derive(Event)]
struct MoveProcessed {
    position: (usize, usize),
    pieces_to_flip: Vec<(usize, usize)>,
    player: ColorPiece,
}

#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::InGame)]
enum TurnState {
    #[default]
    HumanTurn,
    AiWaiting,
    AiThinking,
}

#[derive(Resource)]
struct AiTimer(Timer);

#[derive(Resource)]
struct EndGameAnimation {
    black_to_spawn: usize,
    white_to_spawn: usize,
    spawned_black: usize,
    spawned_white: usize,
    timer: Timer,
}

#[derive(Resource)]
pub struct GameConfig {
    pub show_playable_indicators: bool,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Setup,
    Menu,
    Config,
    InGame,
    EndGame,
    GameOverScreen,
}

fn main() {
    let use_case = UseCase::default();
    let board = use_case.initialize_game_use_case.execute();

    App::new()
        // .insert_resource(WinitSettings::desktop_app())
        .insert_resource(BoardResource(board))
        .insert_resource(UseCaseResource(use_case))
        .insert_resource(AiTimer(Timer::from_seconds(1.0, TimerMode::Once)))
        .insert_resource(GameConfig {
            show_playable_indicators: true,
        })
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
        .add_sub_state::<TurnState>()
        .add_plugins((MenuPlugin, FireworkPlugin))
        .add_systems(Startup, setup_game)
        .add_systems(Update, tick_despawn_timers)
        .add_systems(
            OnEnter(InGame),
            (create_board_instance, create_board_ui).chain(),
        )
        .add_systems(OnExit(InGame), remove_board)
        .add_systems(
            OnEnter(EndGame),
            (create_board_ui, setup_end_game_animation).chain(),
        )
        .add_systems(OnExit(EndGame), remove_board)
        .add_systems(Update, animate_end_game.run_if(in_state(EndGame)))
        .add_observer(check_end_game_observer)
        .add_observer(apply_move)
        .add_observer(execute_player_move)
        .add_systems(Update, show_playable_moves.run_if(in_state(HumanTurn)))
        .add_systems(OnExit(HumanTurn), hide_playable_moves)
        .add_systems(OnEnter(GameOverScreen), setup_game_over_screen)
        .add_systems(OnExit(GameOverScreen), cleanup_game_over)
        .add_systems(
            Update,
            (
                handle_click.run_if(in_state(HumanTurn).and(input_just_pressed(MouseButton::Left))),
                ai_wait_system.run_if(in_state(AiWaiting)),
                ai_play_system.run_if(in_state(AiThinking)),
            )
                .chain(),
        )
        .run();
}

fn check_end_game_observer(
    _trigger: On<MoveProcessed>,
    game_res: Res<BoardResource>,
    use_case: Res<UseCaseResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if use_case
        .0
        .evaluate_game_end_use_case
        .execute(&game_res.0)
        .is_some()
    {
        next_state.set(EndGame);
    }
}

fn apply_move(
    move_processed: On<MoveProcessed>,
    mut commands: Commands,
    mut query: Query<(&CaseUi, &mut Sprite)>,
    board: Single<Entity, With<BoardRoot>>,
    assets: Res<GameAssets>,
) {
    let board_entity = board.entity();

    for (case, mut sprite) in &mut query {
        if move_processed.pieces_to_flip.contains(&(case.x, case.y))
            && let Some(sprite) = sprite.texture_atlas.as_mut()
        {
            sprite.index = if move_processed.player == Black { 1 } else { 0 };
        }
    }
    commands.entity(board_entity).with_children(|parent| {
        add_piece(
            parent,
            move_processed.position.0,
            move_processed.position.1,
            &move_processed.player,
            &assets,
        );
    });
}

fn execute_player_move(
    move_accepted: On<MoveAccepted>,
    mut commands: Commands,
    mut game_res: ResMut<BoardResource>,
    use_case: ResMut<UseCaseResource>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let board = &mut game_res.0;
    let option = use_case
        .0
        .play_move_use_case
        .execute(board, move_accepted.x, move_accepted.y);
    if let Some(flip_pieces) = option {
        commands.trigger(MoveProcessed {
            position: (move_accepted.x, move_accepted.y),
            pieces_to_flip: flip_pieces,
            player: Black,
        });

        if board.player2() {
            next_state.set(AiWaiting);
        }
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
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
    next_state.set(Menu);
}

fn create_board_instance(use_case: ResMut<UseCaseResource>, mut game_res: ResMut<BoardResource>) {
    let board = use_case.0.initialize_game_use_case.execute();
    game_res.0 = board;
}

fn create_board_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_res: Res<BoardResource>,
    assets: Res<GameAssets>,
    state: Res<State<GameState>>,
) {
    let mut parent = commands.spawn((BoardRoot, Transform::default(), Visibility::default()));
    let vertical_segment = meshes.add(Segment2d::new(
        Vec2::new(0f32, -CELL_SIZE * 4f32),
        Vec2::new(0f32, CELL_SIZE * 4f32),
    ));
    let horizontal_segment = meshes.add(Segment2d::new(
        Vec2::new(-CELL_SIZE * 4f32, 0f32),
        Vec2::new(CELL_SIZE * 4f32, 0f32),
    ));

    parent.with_children(|parent| {
        parent.spawn((
            Sprite {
                image: assets.texture_wood.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -1.0),
        ));

        let rectangle = meshes.add(Rectangle::new(CELL_SIZE * 8f32, CELL_SIZE * 8f32));

        let black = Color::linear_rgb(0., 0., 0.);
        let green_reversi = Color::linear_rgb(0.0, 0.4, 0.0);

        let black_color_handle = materials.add(black);
        let green_reversi_color_handle = materials.add(green_reversi);

        parent.spawn((
            Mesh2d(rectangle),
            MeshMaterial2d(green_reversi_color_handle.clone()),
            Transform::from_xyz(0f32, 0f32, 0f32),
        ));

        for i in -4..=4 {
            parent.spawn((
                Mesh2d(vertical_segment.clone()),
                MeshMaterial2d(black_color_handle.clone()),
                Transform::from_xyz(CELL_SIZE * i as f32, 0f32, 1f32),
            ));

            parent.spawn((
                Mesh2d(horizontal_segment.clone()),
                MeshMaterial2d(black_color_handle.clone()),
                Transform::from_xyz(0f32, CELL_SIZE * i as f32, 1f32),
            ));
        }
        for (x, y) in BoardIter::default() {
            if state.get() == &InGame
                && let Some(Case::Piece(color)) = game_res.0.cell(x, y)
            {
                add_piece(parent, x, y, color, &assets);
            }
        }
    });
}

fn handle_click(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
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
            commands.trigger(MoveAccepted { x, y });
        }
    }
}

fn ai_wait_system(
    time: Res<Time>,
    mut ai_timer: ResMut<AiTimer>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    ai_timer.0.tick(time.delta());
    if ai_timer.0.just_finished() {
        next_state.set(AiThinking);
        ai_timer.0.reset();
    }
}

fn ai_play_system(
    mut commands: Commands,
    mut game: ResMut<BoardResource>,
    use_case: ResMut<UseCaseResource>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let board = &mut game.0;
    let use_case = &use_case.0;
    let move_ia = use_case.play_ai_move_use_case.execute(board);

    if let Some(selected_move) = move_ia {
        commands.trigger(MoveProcessed {
            position: selected_move.position(),
            pieces_to_flip: selected_move.pieces_to_flip(),
            player: White,
        });

        if board.player1() {
            next_state.set(HumanTurn);
        } else if board.player2() {
            next_state.set(AiWaiting);
        }
    }
}

fn add_piece(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
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

fn remove_board(query: Query<Entity, With<BoardRoot>>, mut commands: Commands) {
    for input in query {
        commands.entity(input).despawn();
    }
}

fn setup_end_game_animation(mut commands: Commands, board_res: Res<BoardResource>) {
    if let Some(score) = board_res.0.end_of_game() {
        commands.insert_resource(EndGameAnimation {
            black_to_spawn: score.player1(),
            white_to_spawn: score.player2(),
            spawned_black: 0,
            spawned_white: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        });
    }
}

fn animate_end_game(
    mut commands: Commands,
    time: Res<Time>,
    mut animation: ResMut<EndGameAnimation>,
    assets: Res<GameAssets>,
    board_root: Query<Entity, With<BoardRoot>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    animation.timer.tick(time.delta());

    if animation.timer.just_finished()
        && let Some(board_entity) = board_root.iter().next()
    {
        if animation.spawned_black < animation.black_to_spawn {
            let i = animation.spawned_black;
            let x = i % 8;
            let y = i / 8;
            commands.entity(board_entity).with_children(|parent| {
                add_piece(parent, x, y, &Black, &assets);
            });
            animation.spawned_black += 1;
        } else if animation.spawned_white < animation.white_to_spawn {
            let i = animation.spawned_black + animation.spawned_white;
            let x = i % 8;
            let y = i / 8;
            commands.entity(board_entity).with_children(|parent| {
                add_piece(parent, x, y, &White, &assets);
            });
            animation.spawned_white += 1;
        } else {
            commands.remove_resource::<EndGameAnimation>();
            next_state.set(GameOverScreen);
        }
    }
}

#[derive(Component)]
pub struct GameOverRoot;

fn setup_game_over_screen(mut commands: Commands, board_res: Res<BoardResource>) {
    let score = board_res.0.end_of_game().unwrap();
    let black_score = score.player1();
    let white_score = score.player2();

    let result_text = if black_score > white_score {
        "Victoire !"
    } else if white_score > black_score {
        "Defaite..."
    } else {
        "Match Nul !"
    };

    let score_text = format!("Noir: {} - Blanc: {}", black_score, white_score);

    commands
        .spawn((
            GameOverRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(result_text),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(score_text),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    if black_score > white_score {
        // Déclencher le feu d'artifice
        for i in 0..15 {
            commands.spawn((
                Firework {
                    timer: Timer::from_seconds(0.5 + i as f32 * 0.6, TimerMode::Once),
                },
                GameOverRoot,
            ));
        }
    }

    commands.spawn((
        GameOverRoot,
        DespawnTimer(Timer::from_seconds(10.0, TimerMode::Once)),
    ));
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
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
            next_state.set(Menu);
        }
    }
}

fn show_playable_moves(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board_res: Res<BoardResource>,
    board_root: Query<Entity, With<BoardRoot>>,
    config: Res<GameConfig>,
) {
    if !config.show_playable_indicators {
        return;
    }

    let board = &board_res.0;
    let playable_moves = board.available_positions(board.current_player());

    for root in &board_root {
        commands.entity(root).with_children(|parent| {
            let mesh = meshes.add(Circle::new(CELL_SIZE / 4.0));
            // Vert clair semi-transparent
            let material = materials.add(Color::srgba(0.0, 1.0, 0.0, 0.3));

            for (x, y) in playable_moves.clone() {
                parent.spawn((
                    PlayableIndicator,
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(
                        (x as isize - 4) as f32 * CELL_SIZE + CELL_SIZE / 2.,
                        (-(y as isize) + 4) as f32 * CELL_SIZE - CELL_SIZE / 2.,
                        0f32,
                    ),
                ));
            }
        });
    }
}

fn hide_playable_moves(mut commands: Commands, query: Query<Entity, With<PlayableIndicator>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
