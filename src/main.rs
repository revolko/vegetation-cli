mod connection;
mod cli_args;
mod operations;
mod config;

use clap::Parser;

use connection::Connection;
use cli_args::{CliParameters, CLIOperation};
use operations::{register, login, list_plants, create_plant, delete_plant};

fn main() -> () {
    let connection = Connection::build_connection();

    let args = CliParameters::parse();
    match args.operation {
        CLIOperation::ListPlants => list_plants(connection),
        CLIOperation::Login(login_args) => login(login_args, connection),
        CLIOperation::Register(register_args) => register(register_args, connection),
        CLIOperation::CreatePlant(create_plant_args) => create_plant(create_plant_args, connection),
        CLIOperation::DeletePlant(delete_plant_args) => delete_plant(delete_plant_args, connection),
    }
}
