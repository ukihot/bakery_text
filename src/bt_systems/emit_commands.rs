use std::str::FromStr;

use bevy::prelude::*;

use super::handle_inputs::Emitation;
use crate::FocusedSection;

#[derive(Debug)]
pub enum GeneralCommand {
    Ls,
    Mv,
    Shoo,
}

impl FromStr for GeneralCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ls" => Ok(GeneralCommand::Ls),
            "mv" => Ok(GeneralCommand::Mv),
            "shoo" => Ok(GeneralCommand::Shoo),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum PurchasingCommand {
    Order,
}

impl FromStr for PurchasingCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "order" => Ok(PurchasingCommand::Order),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum MixingCommand {
    Mix,
}

impl FromStr for MixingCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mix" => Ok(MixingCommand::Mix),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum CoolingCommand {
    Proof,
    Cool,
}

impl FromStr for CoolingCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "proof" => Ok(CoolingCommand::Proof),
            "cool" => Ok(CoolingCommand::Cool),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ShapingCommand {
    Div,
    Roll,
}

impl FromStr for ShapingCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "div" => Ok(ShapingCommand::Div),
            "roll" => Ok(ShapingCommand::Roll),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum BakingCommand {
    Bake,
}

impl FromStr for BakingCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bake" => Ok(BakingCommand::Bake),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum PackagingCommand {
    Pack,
    Label,
}

impl FromStr for PackagingCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pack" => Ok(PackagingCommand::Pack),
            "label" => Ok(PackagingCommand::Label),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum QualityControlCommand {
    Inspect,
    Report,
}

impl FromStr for QualityControlCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inspect" => Ok(QualityControlCommand::Inspect),
            "report" => Ok(QualityControlCommand::Report),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum StockroomCommand {
    Store,
    Inventory,
}

impl FromStr for StockroomCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "store" => Ok(StockroomCommand::Store),
            "inventory" => Ok(StockroomCommand::Inventory),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum WasteStationCommand {
    Dispose,
}

impl FromStr for WasteStationCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dispose" => Ok(WasteStationCommand::Dispose),
            _ => Err(()),
        }
    }
}

fn handle_purchasing_command(input: &str) {
    if let Ok(cmd) = input.parse::<PurchasingCommand>() {
        eprintln!("Purchasing Command: {:?}", cmd);
    }
}

fn handle_mixing_command(input: &str) {
    if let Ok(cmd) = input.parse::<MixingCommand>() {
        eprintln!("Mixing Command: {:?}", cmd);
    }
}

fn handle_cooling_command(input: &str) {
    if let Ok(cmd) = input.parse::<CoolingCommand>() {
        eprintln!("Cooling Command: {:?}", cmd);
    }
}

fn handle_shaping_command(input: &str) {
    if let Ok(cmd) = input.parse::<ShapingCommand>() {
        eprintln!("Shaping Command: {:?}", cmd);
    }
}

fn handle_baking_command(input: &str) {
    if let Ok(cmd) = input.parse::<BakingCommand>() {
        eprintln!("Baking Command: {:?}", cmd);
    }
}

fn handle_packaging_command(input: &str) {
    if let Ok(cmd) = input.parse::<PackagingCommand>() {
        eprintln!("Packaging Command: {:?}", cmd);
    }
}

fn handle_quality_control_command(input: &str) {
    if let Ok(cmd) = input.parse::<QualityControlCommand>() {
        eprintln!("Quality Control Command: {:?}", cmd);
    }
}

fn handle_stockroom_command(input: &str) {
    if let Ok(cmd) = input.parse::<StockroomCommand>() {
        eprintln!("Stockroom Command: {:?}", cmd);
    }
}

fn handle_waste_station_command(input: &str) {
    if let Ok(cmd) = input.parse::<WasteStationCommand>() {
        eprintln!("Waste Station Command: {:?}", cmd);
    }
}

fn exec_ls() {
    eprintln!("Executing ls command");
}

fn exec_mv() {
    eprintln!("Executing mv command");
}

fn handle_general_command(input: &str) {
    if let Ok(cmd) = input.parse::<GeneralCommand>() {
        eprintln!("General Command: {:?}", cmd);
        match cmd {
            GeneralCommand::Ls => exec_ls(),
            GeneralCommand::Mv => exec_mv(),
            _ => {}
        }
    }
}

pub fn handle_commands(mut events: EventReader<Emitation>, focused_section: Res<FocusedSection>) {
    for ev in events.read() {
        eprintln!("Input: {}", ev.0);

        let parts: Vec<&str> = ev.0.split('_').collect();
        let command = parts.first().unwrap_or(&"");
        let option1 = parts.get(1).copied();
        let option2 = parts.get(2).copied();

        match focused_section.0 {
            0 => handle_purchasing_command(command),
            2 => handle_mixing_command(command),
            3 => handle_cooling_command(command),
            4 => handle_shaping_command(command),
            5 => handle_baking_command(command),
            6 => handle_packaging_command(command),
            7 => handle_quality_control_command(command),
            8 => handle_stockroom_command(command),
            10 => handle_waste_station_command(command),
            _ => handle_general_command(command),
        }

        if let Some(opt1) = option1 {
            eprintln!("Option 1: {}", opt1);
        }
        if let Some(opt2) = option2 {
            eprintln!("Option 2: {}", opt2);
        }
    }
}
