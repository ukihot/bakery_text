use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Wallet {
    pub cash: f64,
    pub receivables: f64,
    pub purchases: f64,
}

impl Wallet {
    pub fn update_cash(&mut self, amount: f64) {
        self.cash += amount;
    }

    pub fn update_receivables(&mut self, amount: f64) {
        self.receivables += amount;
    }

    pub fn update_purchases(&mut self, amount: f64) {
        self.purchases += amount;
    }

    pub fn deduct_cash(&mut self, amount: f64) -> Result<(), &'static str> {
        if self.cash < amount {
            Err("Not enough cash in wallet.")
        } else {
            self.cash -= amount;
            Ok(())
        }
    }
}
