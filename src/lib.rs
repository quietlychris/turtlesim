use bevy::prelude::*;
use serde::*;

use bissel::*;
use bissel::node::Node as BisselNode;
use bissel::host::Host as BisselHost;

// Since Bissel's Host and Node structs don't derive the Component trait by default,
// we use the NewType pattern to wrap them
#[derive(Debug, Component)]
pub struct Host(pub BisselHost);
#[derive(Debug, Component)]
pub struct Node<T: Message>(pub BisselNode<Active, T>);

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct UserInput {
    pub forward: f32,
    pub turn: f32,
}

impl UserInput {
    pub fn forward(mut self, forward: f32) -> Self {
        self.forward = forward;
        self
    }

    pub fn turn(mut self, turn: f32) -> Self {
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
