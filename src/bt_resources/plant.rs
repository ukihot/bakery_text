use bevy::prelude::*;

use crate::bt_components::bread::Bread;

#[derive(Debug, Default)]
pub enum HealthStatus {
    #[default]
    Normal,
    Abnormal,
}

#[derive(Debug, Default)]
pub enum OperatorMode {
    #[default]
    Observer,
    Commander,
}

#[derive(Resource)]
pub struct PurchasingSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
}

impl Default for PurchasingSection {
    fn default() -> Self {
        Self {
            operator: OperatorMode::Commander,
            health_status: HealthStatus::default(),
            has_uninvited_guest: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct PantrySection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub flour: f32,
    pub salt: f32,
    pub sugar: f32,
    pub butter: f32,
    pub yeast: f32,
}

#[derive(Resource, Default)]
pub struct MixingSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub flour: f32,
    pub salt: f32,
    pub sugar: f32,
    pub butter: f32,
    pub yeast: f32,
    pub dough: f32,
}

#[derive(Resource, Default)]
pub struct CoolingSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub dough: f32,
}

#[derive(Resource, Default)]
pub struct ShapingSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub dough: f32,
    pub breads: Vec<Bread>,
}

#[derive(Resource, Default)]
pub struct BakingSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub breads: Vec<Bread>,
}

#[derive(Resource, Default)]
pub struct PackagingSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub breads: Vec<Bread>,
}

#[derive(Resource, Default)]
pub struct QualitySection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub breads: Vec<Bread>,
}

#[derive(Resource, Default)]
pub struct StockroomSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
    pub breads: Vec<Bread>,
}

#[derive(Resource, Default)]
pub struct SalesSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub breads: Vec<Bread>,
}

#[derive(Resource, Default)]
pub struct WasteSection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
    pub has_uninvited_guest: bool,
}

#[derive(Resource, Default)]
pub struct UtilitySection {
    pub operator: OperatorMode,
    pub health_status: HealthStatus,
}

pub trait OperatorControl {
    fn activate(&mut self);
    fn deactivate(&mut self);
}

impl OperatorControl for PurchasingSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for PantrySection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for MixingSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for CoolingSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for ShapingSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for BakingSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for PackagingSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for QualitySection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for StockroomSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for SalesSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for WasteSection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}

impl OperatorControl for UtilitySection {
    fn activate(&mut self) {
        self.operator = OperatorMode::Commander;
    }

    fn deactivate(&mut self) {
        self.operator = OperatorMode::Observer;
    }
}
