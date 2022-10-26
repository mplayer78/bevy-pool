use std::f32::consts::PI;

use bevy::{
    input::keyboard::{self, KeyboardInput},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(cue_physics)
        .add_system(display_events)
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
struct Balls;

#[derive(Component)]
struct GameBall;

#[derive(Component)]
struct CueTip {
    has_contacted: bool,
}

fn setup_physics(mut commands: Commands) {
    let h_edge = Collider::cuboid(TABLE_SIZE.0 / 2.0, EDGE_WIDTH / 2.0);
    let v_edge = Collider::cuboid(EDGE_WIDTH / 2.0, TABLE_SIZE.1 / 2.0);
    /* Create the ground. */
    for ind in vec![-1.0, 1.0] {
        commands
            .spawn()
            .insert(h_edge.clone())
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                0.0,
                (TABLE_SIZE.1 + EDGE_WIDTH) / 2.0 * ind,
                0.0,
            )));
        commands
            .spawn()
            .insert(v_edge.clone())
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                (TABLE_SIZE.0 + EDGE_WIDTH) / 2.0 * ind,
                0.0 * ind,
                0.0,
            )));
    }

    /* Create the bouncing ball. */
    const EDGE_TO_STRING: f32 = 22.0 * 2.54;
    const BALL_TOLERANCE: f32 = 0.1;

    commands
        .spawn()
        .insert(CueBall)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_DIAMETER / 2.0))
        .insert(Restitution::coefficient(0.95))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.1,
            angular_damping: 0.1,
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -TABLE_SIZE.1 / 4.0, 0.0)));

        let centre_to_string = TABLE_SIZE.1 / 2.0 - EDGE_TO_STRING;
        for row in 0..5 {
            let v_offset = centre_to_string + ((row - 2) as f32) * (PI / 3.0).sin() * BALL_DIAMETER + BALL_TOLERANCE;
            let h_offset_initial = -(row as f32) / 2.0 * BALL_DIAMETER;
            for placement in 0..row + 1 {
                let ball_x = h_offset_initial + (placement as f32) * BALL_DIAMETER + BALL_TOLERANCE;
                commands.spawn()
                    .insert(GameBall)
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::ball(BALL_DIAMETER / 2.0))
                    .insert(Restitution::coefficient(0.95))
                    .insert(GravityScale(0.0))
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(ball_x, v_offset, 0.0)))                    
                    .insert(Damping {
                        linear_damping: 0.1,
                        angular_damping: 0.1,
                    });
            };
        }
        
    commands
        .spawn()
        .insert(CueTip {
            has_contacted: false,
        })
        .insert(ExternalForce::default())
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 1.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(0.0))
        .insert(Restitution::coefficient(0.95))
        .insert(AdditionalMassProperties::Mass(100.0))
        .insert(Damping {
            linear_damping: 0.3,
            angular_damping: 0.1,
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -60.0, 0.0)));
}

fn cue_physics(
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut cue_query: Query<(Entity, &mut ExternalForce, &mut Velocity, &mut CueTip)>,
    mut cue_ball_query: Query<Entity, With<CueBall>>,
) {
    let (cue_entity, mut cue_force, mut cue_velocity, mut cue) = cue_query.single_mut();
    let mut ball_entity = cue_ball_query.single_mut();

    if keyboard_input.pressed(KeyCode::Space) {
        // cue_force.force = Vec2::new(0.0, 10000.0);
    }

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(handle1, handle2, _) = collision_event {
            if (*handle1 == ball_entity && *handle2 == cue_entity)
                || (*handle2 == ball_entity && *handle1 == cue_entity)
            {
                cue.has_contacted = true;
            }
        }
    }

    if cue.has_contacted && cue_velocity.linvel.length() > 0.0 {
        cue_force.force = cue_velocity.linvel * 1000.0 * -1.0;
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
