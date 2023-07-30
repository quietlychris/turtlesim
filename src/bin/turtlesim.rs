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

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins.build()
            .disable::<bevy::log::LogPlugin>(),
            
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, meadow_host)
        .add_systems(Startup, meadow_ui_node)
        .add_systems(Startup, meadow_position_node)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_asset)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, meadow_user_input)
        // .add_systems(Update, turtle_movement_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0))
    ));

    /* Create the bouncing ball. */
    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)),
            RigidBody::Dynamic,
            Collider::ball(50.0),
            Restitution::coefficient(0.7)
        ));

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
        .spawn((
            sprite_bundle,
            turtle,
            RigidBody::Dynamic,
            Collider::ball(40.0),
            Restitution::coefficient(0.7)
        ));
        
}

// Create a Host where Nodes can exchange information
fn meadow_host(mut commands: Commands) {
    
    // Generate certificates for QUIC 
    // meadow::generate_certs().unwrap();
    // Setup our meadow host
    let meadow_host: MeadowHost = HostConfig::default()// sled DBs allow persistence across reboots
        .with_udp_config(None)
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
    let meadow_node = NodeConfig::<Tcp, UserInput>::new("TURTLESIM_UI")
        .topic("user_input")
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
    let meadow_node = NodeConfig::<Tcp, Position>::new("TURTLESIM_POS")
        .topic("position")
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
) {
    let ui_node = node_query.single_mut();

    let mut user_input: UserInput = match ui_node.0.request() {
        Ok(val) => {
            // info!("We're all turtles!");
            val
        }
        Err(e) => {
            error!("{:?}", e);
            UserInput::default()
        }
    };
    let accel = 0.1;
    if keyboard_input.pressed(KeyCode::Left) {
        user_input.turn += accel;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        user_input.turn -= accel;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        user_input.forward -= accel;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        user_input.forward += accel;
    }
    // println!("going to publish: {:?}", &user_input);

    ui_node.0.publish(user_input).unwrap();
}

/// The meadow Node now requests the UserInput from the Host, and derives the desired
/// forward or rotational motions, then applies those transformations to the Turtle's
/// on-screen sprite representation. In this case, we're using the same Node to both
/// publish and request information, but this can be done equivalently using
fn turtle_movement_system(
    time: Res<Time>,
    // keyboard_input: Res<Input<KeyCode>>,
    mut turtle_query: Query<(&mut Turtle, &mut Transform, &GlobalTransform)>,
    mut ui_node_query: Query<&mut Node<UserInput>>,
    mut position_node_query: Query<&mut Node<Position>>,
) {
    let delta = time.delta_seconds();
    let ui_node = ui_node_query.single_mut();
    let position_node = position_node_query.single_mut();

    let mut movement = UserInput::default();
    match ui_node.0.request() {
        Ok(val) => {
            // println!("{:?}", &val);
            movement = val;
            // After reading the turtle's input, reset the user_input data as (0,0)
            // node.0.publish_to("user_input", UserInput::default());
        }
        Err(e) => println!("Error: {}", e),
    };

    let (turtle, mut transform, global_transform) = turtle_query.single_mut();

    // Do rotation modifications
    let rotation = Quat::from_rotation_z(movement.turn as f32 * delta);
    transform.rotate(rotation);

    // Modify the turtle's speed!
    // turtle.velocity.y += (y * 20.0);

    let euler = transform.rotation.to_euler(EulerRot::XYZ);
    // dbg!(euler);
    let heading = euler.2; // use the Z angle to calculate direction
                           // sin/cos may need to be switched depending on the initial orientation of the sprite texture
    let x_movement = turtle.velocity.x * movement.forward * f32::cos(heading) * delta; // * delta
    let y_movement = turtle.velocity.y * movement.forward * f32::sin(heading) * delta; // * delta

    transform.translation.x += x_movement;
    transform.translation.y += y_movement;

    /*
    println!(
        "x: {}, y: {}, yaw: {}",
        global_transform.translation.x as f32, global_transform.translation.y as f32,
        global_transform.rotation.to_euler(EulerRot::XYZ).2.to_degrees()
    );*/
    let (_scale, rotation, translation) = global_transform.to_scale_rotation_translation();

    let position = Position {
        x: translation.x,
        y: translation.y,
        yaw: rotation.to_euler(EulerRot::XYZ).2.to_degrees(),
    };

    position_node.0.publish(position).unwrap();

    // Set min/max boundaries along the x- and y-axis
    transform.translation.x = transform.translation.x.min(400.0).max(-400.0);
    transform.translation.y = transform.translation.y.min(250.0).max(-250.0);
}
