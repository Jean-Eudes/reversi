use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use reversi_core::application::use_case::UseCase;
use reversi_core::domain::board::ColorPiece::Black;
use reversi_core::domain::board::{Board, BoardIter, Case, ColorPiece};

const CELL_SIZE: f32 = 60f32;

#[derive(Resource)]
struct BoardResource(Board);

#[derive(Component, Debug)]
struct CaseUi {
    x: usize,
    y: usize,
}

#[derive(Message)]
pub struct MoveAccepted {
    pub x: usize,
    pub y: usize,
}

fn main() {
    let use_case = UseCase::default();
    let board = use_case.initialize_game_use_case.execute();

    let app = App::new()
        //.insert_resource(WinitSettings::desktop_app())
        .insert_resource(BoardResource(board))
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
        .add_systems(Startup, create_board)
        .add_message::<MoveAccepted>()
        //.add_systems(Update, refresh_board)
        .add_systems(Update, (handle_click, execute_player_move).chain())
        .run();
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_res: ResMut<BoardResource>,
) {
    let vertical_segment = meshes.add(Segment2d::new(
        Vec2::new(0f32, -CELL_SIZE * 4f32),
        Vec2::new(0f32, CELL_SIZE * 4f32),
    ));
    let horizontal_segment = meshes.add(Segment2d::new(
        Vec2::new(-CELL_SIZE * 4f32, 0f32),
        Vec2::new(CELL_SIZE * 4f32, 0f32),
    ));
    commands.spawn(Camera2d);

    let color = Color::linear_rgb(0., 0., 0.);
    //let color = Color::hsl(300f32, 0.95, 0.7);

    for i in -4..=4 {
        let handle = materials.add(color);
        commands.spawn((
            Mesh2d(vertical_segment.clone()),
            MeshMaterial2d(handle.clone()),
            Transform::from_xyz(CELL_SIZE * i as f32, 0f32, 0f32),
        ));

        commands.spawn((
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

fn execute_player_move(
    mut commands: Commands,
    mut game_res: ResMut<BoardResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&CaseUi, &mut MeshMaterial2d<ColorMaterial>)>,
    mut message_reader: MessageReader<MoveAccepted>,
) {
    for move_accepted in message_reader.read() {
        let color = game_res.0.current_player().color();
        let option = game_res.0.place(move_accepted.x, move_accepted.y);
        if let Some(flipped_case) = option {
            for content in &query {
                if flipped_case.contains(&(content.0.x, content.0.y))
                    && let Some(mat) = materials.get_mut(&content.1.0)
                {
                    mat.color = if color == ColorPiece::Black {
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
                move_accepted.x,
                move_accepted.y,
                &color,
            );
        }
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
