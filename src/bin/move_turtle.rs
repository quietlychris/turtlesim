use bissel::*;
use turtlesim::{Position, UserInput};

use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // Build a Node for controlling the UserInput
    let mut ui_node: Node<UserInput> = NodeConfig::new("MOVE_TURTLE_UI")
        .topic("user_input")
        .build()?;
    // Build a Node for getting Position updates
    let mut position_node: Node<Position> = NodeConfig::new("MOVE_TURTLE_POS")
        .topic("position")
        .build()?;
    // Connect both Nodes
    ui_node.connect()?;
    position_node.connect()?;

    // If there's an existing position on the turtle, get it
    // Otherwise, assume that it starts at the origin
    let mut position = match position_node.request() {
        Ok(position) => position,
        _ => Position::default(),
    };

    // Rotate the turtle to -90 degrees (down)
    while position.yaw > -90.0 && position.yaw <= 0.0 {
        position = position_node.request()?;
        println!("position: {:?}", position);
        let input = UserInput {
            forward: 0.0,
            turn: -1.0,
        };
        ui_node.publish(input)?;
    }

    // Move the turn to the bottom of the screen
    while position.y > -120.0 {
        ui_node.publish(UserInput::default().forward(1.0))?;
        position = position_node.request()?;
        println!("position: {:?}", position);
    }

    // Rotate the turtle back to the original heading
    while position.yaw < 0.0 {
        ui_node.publish(UserInput::default().turn(1.0))?;
        position = position_node.request()?;
        println!("position: {:?}", position);
    }

    // Drive the turtle in a circle for 6.5 seconds
    let now = Instant::now();
    while now.elapsed().as_millis() < 6_500 {
        ui_node.publish(UserInput::default().turn(1.0).forward(1.0))?;
        position = position_node.request()?;
        println!("position: {:?}", position);
    }

    // Stop the turtle
    println!("Stopping the turtle");
    ui_node.publish(UserInput::default())?;

    Ok(())
}
