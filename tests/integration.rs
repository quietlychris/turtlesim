use bissel::*;
use turtlesim::{Position, UserInput};

#[test]
fn test_user_input_and_position() {
    // Get the host up and running
    let mut host: Host = HostConfig::new("lo")
        .socket_num(25_000)
        .store_filename("store")
        .build()
        .unwrap();
    host.start().unwrap();
    println!("Host should be running in the background");

    let ui_node = NodeConfig::<UserInput>::new("TEST_UI")
        .topic("test_user_input")
        .build()
        .unwrap()
        .connect()
        .unwrap();

    let position_node = NodeConfig::<Position>::new("TEST_POS")
        .topic("test_position")
        .build()
        .unwrap()
        .connect()
        .unwrap();
        
    println!("- Both nodes successfully connected");

    let user_input = UserInput {
        turn: 0.,
        forward: 0.
    };

    ui_node.publish(user_input.clone()).unwrap();
    let output: UserInput = ui_node.request().unwrap();
    println!("deserialized output: {:?}", output);
    assert_eq!(user_input, output);

    let position = Position {
        x: 1.0,
        y: 2.0,
        yaw: 45.0,
    };
    position_node.publish(position.clone()).unwrap();
    let output = position_node.request().unwrap();
    println!("deserialized output: {:?}", position);
    assert_eq!(position, output);
    host.stop().unwrap();
}
