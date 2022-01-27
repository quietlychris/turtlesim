use bevy::prelude::*;
use serde::*;

use bissel::host::Host as BisselHost;
use bissel::node::Message;
use bissel::node::Node as BisselNode;

// Since Bissel's Host and Node structs don't derive the Component trait by default,
// we use the NewType pattern to wrap them
#[derive(Debug, Component)]
pub struct Host(pub BisselHost);
#[derive(Debug, Component)]
pub struct Node<T: Message>(pub BisselNode<T>);

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct UserInput {
    pub forward: isize,
    pub turn: isize,
}

impl UserInput {
    pub fn forward(mut self, forward: isize) -> Self {
        self.forward = forward;
        self
    }

    pub fn turn(mut self, turn: isize) -> Self {
        self.turn = turn;
        self
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
}
