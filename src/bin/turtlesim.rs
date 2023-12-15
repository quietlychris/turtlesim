use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32;

use meadow::{Host as MeadowHost, HostConfig, Node as MeadowNode, NodeConfig, *};
// These are NewType wrappers around the meadow Host and Node structs
use turtlesim::{Host, Node};
use turtlesim::{Position, UserInput};

const SPRITE_SIZE: f32 = 50.0;

#[derive(Debug, Default, Component)]
struct Turtle {
    velocity: Vec3,
}

#[derive(Debug, Default, Component)]
struct Paused(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<bevy::log::LogPlugin>())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, meadow_host)
        .add_systems(Startup, meadow_ui_node)
        .add_systems(Startup, meadow_position_node)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_asset)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, meadow_user_input)
        .add_systems(Update, pause)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    // Set up the default view
    commands.spawn(Camera2dBundle::default());
    // Keep track of whether the simulation is paused or not
    commands.spawn(Paused(false));
}

fn setup_physics(mut commands: Commands, mut time: ResMut<Time<Virtual>>) {
    /* Create the ground. */
    commands
        .spawn(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Collider::cuboid(500.0, 50.0));

    /* Create the bouncing ball. */
    commands
        .spawn(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(SPRITE_SIZE))
        .insert(Restitution::coefficient(0.7));

    // Set a "time warp" on the clock that our physics uses
    // If we set this below 1.0, we will go very slow
    time.set_relative_speed(3.0);
}

fn setup_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the Bevy icon
    let texture_handle = asset_server.load("turtle.png");

    // Defines the Sprite itself, with Sprite {color, flip_x, flip_y, custom_size} fields
    // Can resize a sprite's texture if the default is too small or large
    let sprite = Sprite {
        custom_size: Some(Vec2::new(1.5, 1.0) * SPRITE_SIZE),
        ..Default::default()
    };

    // Sprite bundle is the on-screen representation of a given icon
    let sprite_bundle = SpriteBundle {
        sprite,
        texture: texture_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    // Create a new Turtle!
    let turtle = Turtle {
        velocity: Vec3::new(200.0, 200.0, 0.0),
    };

    commands
        .spawn(sprite_bundle)
        .insert(turtle)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        });
}

// Create a Host where Nodes can exchange information
fn meadow_host(mut commands: Commands) {
    // Setup our meadow host
    let meadow_host: MeadowHost = HostConfig::default() // sled DBs allow persistence across reboots
        .build()
        .expect("Couldn't create a Host");

    let mut host = Host(meadow_host);
    host.0.start().unwrap();

    commands.spawn(host);
}

// Create a node for managing UserInput
fn meadow_ui_node(mut commands: Commands) {
    // Sleep for a second while setting up to allow the Host to fully get setup
    std::thread::sleep(std::time::Duration::from_millis(1_000));
    let meadow_node = NodeConfig::<Tcp, UserInput>::new("user_input")
        .build()
        .unwrap()
        .activate()
        .unwrap();
    let ui_node = Node(meadow_node);
    // Each node establishes a TCP connection with central host

    commands.spawn(ui_node);
}

// Create a node for managing UserInput
fn meadow_position_node(mut commands: Commands) {
    // Sleep for a second while setting up to allow the Host to fully get setup
    std::thread::sleep(std::time::Duration::from_millis(1_000));
    let meadow_node = NodeConfig::<Tcp, Position>::new("position")
        .build()
        .unwrap()
        .activate()
        .unwrap();
    let position_node = Node(meadow_node);
    // Each node establishes a TCP connection with central host

    commands.spawn(position_node);
}

/// Using the Bevy-native system, we get the input of the arrow keys, and use them to
/// form a UserInput struct, which the Node then publishes to the meadow Host
fn meadow_user_input(
    mut node_query: Query<&mut Node<UserInput>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut turtle_query: Query<(
        &mut Turtle,
        &mut RigidBody,
        &mut ExternalForce,
        &mut ExternalImpulse,
    )>,
) {
    let ui_node = node_query.single_mut();
    let (turtle, mut body, mut force, mut impulse) = turtle_query.single_mut();

    let mut user_input: UserInput = match ui_node.0.request() {
        Ok(msg) => {
            // info!("We're all turtles!");
            msg.data
        }
        Err(e) => {
            error!("{:?}", e);
            UserInput::default()
        }
    };
    if keyboard_input.pressed(KeyCode::Left) {
        // user_input.turn += accel;
        // impulse.torque_impulse = 0.4;
        force.torque += -0.01;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        force.torque += 0.01;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        force.force -= Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        force.force += Vec2::new(0.0, 1.0);
    }
    // println!("going to publish: {:?}", &user_input);

    ui_node.0.publish(user_input).unwrap();
}

fn pause(
    keyboard_input: Res<Input<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    mut paused_query: Query<&mut Paused>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let mut paused = paused_query.single_mut();
        if paused.0 == false {
            time.pause();
            paused.0 = true;
        } else {
            time.unpause();
            paused.0 = false;
        }
    }
}
