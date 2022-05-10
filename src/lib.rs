use bevy::prelude::*;
use serde::*;

use meadow::host::Host as meadowHost;
use meadow::node::Node as meadowNode;
use meadow::*;

// Since meadow's Host and Node structs don't derive the Component trait by default,
// we use the NewType pattern to wrap them
#[derive(Debug, Component)]
pub struct Host(pub meadowHost);
#[derive(Debug, Component)]
pub struct Node<T: Message>(pub meadowNode<Active, T>);

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
