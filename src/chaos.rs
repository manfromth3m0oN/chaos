use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use std::f32;

const POINT_SIZE: f32 = 2.;
const TRIANGLE_POINTS: f32 = 350.;

pub struct ChaosGame;

impl Plugin for ChaosGame {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board).add_system(play);
    }
}

#[derive(Component)]
struct Vertex;

#[derive(Component)]
struct Last;

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let camera = Camera2dBundle::default();
    commands.spawn_bundle(camera);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                TRIANGLE_POINTS,
                -(TRIANGLE_POINTS),
                0.,
            )),
            ..default()
        })
        .insert(Vertex);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                -(TRIANGLE_POINTS),
                -(TRIANGLE_POINTS),
                0.,
            )),
            ..default()
        })
        .insert(Vertex);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., TRIANGLE_POINTS, 0.)),
            ..default()
        })
        .insert(Vertex)
        .insert(Last);

    println!("Created triangle");
}

// Draw a point at the midpoint of the last point and a random vertex
fn play(
    mut commands: Commands,
    points_query: Query<(Entity, &Transform), With<Last>>,
    vertex_query: Query<&Transform, With<Vertex>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let point = points_query.single();
    let vertexes = vertex_query.iter().collect::<Vec<&Transform>>();

    let mut rng = rand::thread_rng();

    let vertex_t = vertexes.get(rng.gen_range(0..vertexes.len())).unwrap();

    let vertex = vertex_t.translation;
    let point_vec = point.1.translation;

    let mid: Vec3 = vertex.lerp(point_vec, 0.5);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(POINT_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(mid),
            ..default()
        })
        .insert(Last);

    commands.entity(point.0).remove::<Last>();
}
