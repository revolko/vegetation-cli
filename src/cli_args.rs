use serde::Serialize;
use clap::{Parser, Subcommand, Args};

#[derive(Debug, Args)]
pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Args)]
pub struct RegisterArgs {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Args, Serialize)]
pub struct CreatePlantArgs {
    pub name: String,
    pub description: String,
    pub last_watered: String,
    pub water_frequency_summer: u32,
    pub water_frequency_winter: u32,
    pub watering_type: String,
    pub drought_tolerance: String,
    pub light_requirements: String,
}

#[derive(Debug, Subcommand)]
pub enum CLIOperation {
    Register(RegisterArgs),
    Login(LoginArgs),
    ListPlants,
    CreatePlant(CreatePlantArgs),
}

#[derive(Debug, Parser)]
pub struct CliParameters {
    #[clap(subcommand)]
    pub operation: CLIOperation,  // verbose option
}
