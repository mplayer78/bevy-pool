use bevy::{prelude::*, input::keyboard::{self, KeyboardInput}};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(cue_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera2dBundle::new_with_far(500.0));
}

const BALL_DIAMETER: f32 = 5.715;
const TABLE_SIZE: (f32, f32) = (112.0, 224.0);
const EDGE_WIDTH: f32 = 19.0;

#[derive(Component)]
struct CueBall;

#[derive(Component)]
struct CueTip;

fn setup_physics(mut commands: Commands) {
    let h_edge = Collider::cuboid(TABLE_SIZE.0 / 2.0, EDGE_WIDTH / 2.0);
    let v_edge = Collider::cuboid(EDGE_WIDTH / 2.0, TABLE_SIZE.1 / 2.0);
    /* Create the ground. */
    for ind in vec![-1.0, 1.0] {
        commands
            .spawn()
            .insert(h_edge.clone())
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, (TABLE_SIZE.1 + EDGE_WIDTH) / 2.0 * ind, 0.0)));
        commands
            .spawn()
            .insert(v_edge.clone())
            .insert_bundle(TransformBundle::from(Transform::from_xyz((TABLE_SIZE.0 + EDGE_WIDTH) / 2.0 * ind, 0.0 * ind, 0.0)));
    }

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(CueBall)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_DIAMETER / 2.0))
        .insert(Restitution::coefficient(0.95))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.1,
            angular_damping: 0.1
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -40.0, 0.0)));
        
    commands
        .spawn()
        .insert(CueTip)
        .insert(ExternalForce::default())
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 1.0))
        .insert(Sensor(true))
        .insert(GravityScale(0.0))
        .insert(Restitution::coefficient(0.95))
        .insert(AdditionalMassProperties::Mass(100.0))
        .insert(Damping {
            linear_damping: 0.3,
            angular_damping: 0.1
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -60.0, 0.0)));
}

fn cue_physics(
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cue_query: Query<(Entity, &mut ExternalForce), With<CueTip>>,
    mut cue_ball_query: Query<Entity, With<CueBall>>,
) {
    for (cue_entity, mut cue_force) in cue_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            cue_force.force = Vec2::new(0.0, 10000.0);
        }
        for ball_entity in cue_ball_query.iter() {
            if rapier_context.intersection_pair(cue_entity, ball_entity) == Some(true) {
                println!("Yep");
                cue_force.force = Vec2::ZERO
            }
        }
    }
}