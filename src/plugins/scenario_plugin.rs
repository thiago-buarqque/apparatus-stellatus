use bevy::prelude::*;

pub const CELL: f32 = 32.0;
pub const BOARD_WIDTH: usize = 500;
pub const BOARD_HEIGHT: usize = 500;

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, (setup_board_cells).chain())
            .add_systems(Update, (paint_board_cells,).chain());
    }
}

fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window>,
) {
    commands.spawn(Camera2d);

    clear_color.0 = Color::srgba_u8(73, 77, 100, 255);

    let Ok(mut window) = window_query.single_mut() else {
        return;
    };

    window.title = String::from("Apparatus Stellatus");
}

// Constants for piece starting position
pub const PIECE_START_X: i32 = (BOARD_WIDTH as i32 / 2) - 2; // Centred for 4-wide shapes
pub const PIECE_START_Y: i32 = 18; // Two rows from the top

/// Updates the visual representation of the game grid.
fn update_grid(mut gizmos: Gizmos) {
    let color = Color::srgba_u8(73, 77, 100, 255);

    // Draw a rectangle representing the board boundaries
    gizmos.rect_2d(
        Isometry2d::IDENTITY, // Position at origin
        Vec2::new(CELL * BOARD_WIDTH as f32, CELL * BOARD_HEIGHT as f32),
        color,
    );
}

#[derive(Component)]
struct GridBackground;

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let quad = meshes.add(Rectangle::new(
        BOARD_WIDTH as f32 * CELL,
        BOARD_HEIGHT as f32 * CELL,
    ));

    let mat = materials.add(ColorMaterial::from(Color::srgba_u8(73, 77, 100, 255)));

    commands.spawn((
        Mesh2d::from(quad),
        MeshMaterial2d(mat),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GridBackground,
        GlobalTransform::default(),
    ));
}

fn paint_board_cells(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&GridCell, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    // Define what the “empty” color should be
    let empty_tint = Color::srgba_u8(255, 77, 100, 255);

    for (grid_cell, mat2d) in &mut query {
        let x = grid_cell.pos.x as usize;
        let y = grid_cell.pos.y as usize;

        // board.cell(x,y) should be: Some(color) if occupied, None if empty
        // (Assuming you have `pub fn cell(&self, x:usize, y:usize) -> Option<Color>`)
        let target_color = empty_tint;

        // mat2d.0 is the Handle<ColorMaterial>; we look it up and tint it
        //if let Some(material) = materials.get_mut(&mat2d.0) {
        //    // Only update if different, to avoid extra GPU churn
        //    if material.color != target_color {
        //        material.color = target_color;
        //    }
        //}
    }
}

fn setup_board_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let terrain = asset_server.load("textures/terrain.png");
    let quad = meshes.add(Rectangle::new(CELL, CELL)); // one mesh handle
    let mut bundles = Vec::with_capacity(BOARD_WIDTH * BOARD_HEIGHT);

    let origin = Vec2::new(
        -(BOARD_WIDTH as f32 * CELL) * 0.5,
        -(BOARD_HEIGHT as f32 * CELL) * 0.5,
    );

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let centre =
                origin + Vec2::new(x as f32 * CELL + CELL * 0.5, y as f32 * CELL + CELL * 0.5);
            let translation = Vec3::new(centre.x, centre.y, 1.0);

            bundles.push((
                Mesh2d::from(quad.clone()),
                Transform::from_translation(translation),
                GlobalTransform::default(),
                Visibility::default(),
                Sprite::from_image(terrain.clone()),
                GridCell {
                    pos: IVec2::new(x as i32, y as i32),
                },
            ));
        }
    }

    commands.spawn_batch(bundles);
}

#[derive(Component)]
pub struct GridCell {
    /// (x,y) indices in board coordinates (0 ≤ x < BOARD_WIDTH, 0 ≤ y < BOARD_HEIGHT)
    pub pos: IVec2,
}
