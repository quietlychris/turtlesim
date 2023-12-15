use meadow::*;
use turtlesim::{Position, UserInput};

#[test]
fn test_user_input_and_position() {
    // Get the host up and running
    let mut host: Host = HostConfig::default().build().unwrap();
    host.start().unwrap();
    println!("Host should be running in the background");

    let ui_node = NodeConfig::<Tcp, UserInput>::new("test_user_input")
        .build()
        .unwrap()
        .activate()
        .unwrap();

    let position_node = NodeConfig::<Tcp, Position>::new("test_position")
        .build()
        .unwrap()
        .activate()
        .unwrap();

    println!("- Both nodes successfully connected");

    let user_input = UserInput {
        turn: 0.,
        forward: 0.,
    };

    ui_node.publish(user_input.clone()).unwrap();
    let output = ui_node.request().unwrap();
    println!("deserialized output: {:?}", output.data);
    assert_eq!(user_input, output.data);

    let position = Position {
        x: 1.0,
        y: 2.0,
        yaw: 45.0,
    };
    position_node.publish(position.clone()).unwrap();
    let output = position_node.request().unwrap();
    println!("deserialized output: {:?}", position);
    assert_eq!(position, output.data);
    host.stop().unwrap();
}
