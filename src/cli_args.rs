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

#[derive(Debug, Subcommand)]
pub enum CLIOperation {
    Register(RegisterArgs),
    Login(LoginArgs),
    ListPlants,
}

#[derive(Debug, Parser)]
pub struct CliParameters {
    #[clap(subcommand)]
    pub operation: CLIOperation,  // verbose option
}
