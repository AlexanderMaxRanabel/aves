use bevy::{
    prelude::*,
};

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(pub Vec2);
