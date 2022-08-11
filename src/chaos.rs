use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

const POINT_SIZE: f32 = 2.;

pub struct ChaosGame;

impl Plugin for ChaosGame {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board).add_system(play);
    }
}

#[derive(Component)]
struct Vertex(bool);

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Make equalateral triangle
    // To calculate the distance a vertex should be from the center the expression is:
    // sqrt(3)/4 * a
    // Where a is the length of a side

    const VERTICE_LENGTH: f64 = 1000.0;

    let center_distance: f64 = VERTICE_LENGTH * (3.0_f64.sqrt() / 4.0);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                0.,
                (center_distance as f32) - 200.,
                0.,
            )),
            ..default()
        })
        .insert(Vertex(true));

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(center_distance as f32, -200., 0.)),
            ..default()
        })
        .insert(Vertex(true));

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-(center_distance) as f32, -200., 0.)),
            ..default()
        })
        .insert(Vertex(true));

    println!("Created triangle");
}

// Draw a point at the midpoint of the last point and a random vertex
fn play(
    mut commands: Commands,
    points_query: Query<&Transform, Without<Vertex>>,
    vertex_query: Query<&Transform, With<Vertex>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let points = points_query.iter().collect::<Vec<&Transform>>();
    let vertexes = vertex_query.iter().collect::<Vec<&Transform>>();

    let mut rng = rand::thread_rng();

    let vertex_t = vertexes.get(rng.gen_range(0..vertexes.len())).unwrap();
    let point_t: &Transform;
    if points.len() == 0 {
        point_t = vertexes.get(rng.gen_range(0..vertexes.len())).unwrap();
    } else {
        point_t = points.get(rng.gen_range(0..points.len())).unwrap();
    }

    let vertex = vertex_t.translation;
    let point = point_t.translation;

    let mut mid: Vec3 = vertex.lerp(point, 0.5);
    mid.x = mid.x.round();
    mid.y = mid.y.round();

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(mid),
        ..default()
    });
}
