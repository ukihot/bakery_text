use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub enum BreadName {
    Baguette,
    Ciabatta,
    Pretzel,
    Koppe,
}

#[derive(Debug)]
pub enum BreadStatus {
    Unfermented,
    FirstFermentationComplete,
    SecondFermentationComplete,
    BakingComplete,
    CoolingComplete,
    PackagingComplete,
}

#[derive(Debug, Component)]
pub struct Bread {
    pub id: Uuid,
    pub name: BreadName,
    pub selling_price: f32,
    pub expiration: u8, // seconds
    pub is_genuine: bool,
    pub status: BreadStatus,
}

impl Bread {
    pub fn new(name: BreadName) -> Self {
        let (selling_price, expiration) = match name {
            BreadName::Baguette => (1.0, 254),
            BreadName::Ciabatta => (1.6, 157),
            BreadName::Pretzel => (2.6, 97),
            BreadName::Koppe => (4.2, 60),
        };

        Bread {
            id: Uuid::new_v4(),
            name,
            selling_price,
            expiration,
            is_genuine: true,
            status: BreadStatus::Unfermented,
        }
    }
}
