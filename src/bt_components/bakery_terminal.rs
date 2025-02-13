use bevy::prelude::*;

use super::bread::Bread;

#[derive(Debug, Component)]
pub struct ModalComponet;

/// BakeryTerminal component
#[derive(Component, Default)]
pub struct BakeryTerminal {
    pub id: u8,
    pub input_buffer: String,
    pub history: Vec<String>,
    pub status: HealthStatus,
}

#[derive(Debug, Default)]
pub enum HealthStatus {
    #[default]
    Normal,
    Abnormal,
}

#[derive(Debug, Default, Component)]
pub enum OperatorMode {
    #[default]
    Observer,
    Commander,
}

#[derive(Debug, Default, Component)]
pub struct Repository {
    pub flour: Option<f32>,
    pub salt: Option<f32>,
    pub sugar: Option<f32>,
    pub butter: Option<f32>,
    pub yeast: Option<f32>,
    pub dough: Option<f32>,
    pub bread: Option<Vec<Bread>>,
}

impl Repository {
    pub fn new_raw_only() -> Self {
        Self {
            flour: Some(0.0),
            salt: Some(0.0),
            sugar: Some(0.0),
            butter: Some(0.0),
            yeast: Some(0.0),
            dough: None,
            bread: None,
        }
    }

    pub fn new_raw_with_dough() -> Self {
        Self {
            flour: Some(0.0),
            salt: Some(0.0),
            sugar: Some(0.0),
            butter: Some(0.0),
            yeast: Some(0.0),
            dough: Some(0.0),
            bread: None,
        }
    }

    pub fn new_all() -> Self {
        Self {
            flour: Some(0.0),
            salt: Some(0.0),
            sugar: Some(0.0),
            butter: Some(0.0),
            yeast: Some(0.0),
            dough: Some(0.0),
            bread: Some(vec![]),
        }
    }

    pub fn new_dough_with_bread() -> Self {
        Self {
            flour: None,
            salt: None,
            sugar: None,
            butter: None,
            yeast: None,
            dough: Some(0.0),
            bread: Some(vec![]),
        }
    }

    pub fn new_bread_only() -> Self {
        Self {
            flour: None,
            salt: None,
            sugar: None,
            butter: None,
            yeast: None,
            dough: None,
            bread: Some(vec![]),
        }
    }
}
