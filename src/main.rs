mod connection;
mod cli_args;
mod operations;

use std::io::{Write, self};
use std::env;
use clap::Parser;

use connection::{Connection};
use cli_args::{CliParameters, CLIOperation};
use operations::{register, login};

fn main() -> Result<(), Box<reqwest::Error>> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let connection = Connection::build_connection();

    let args = CliParameters::parse();
    match args.operation {
        CLIOperation::ListPlants => writeln!(handle, "got list").unwrap(),
        CLIOperation::Login(login_args) => login(login_args, connection),
        CLIOperation::Register(register_args) => register(register_args, connection),
    }

    let token = match env::var("VEGETATION_TOKEN") {
        Ok(value) => value,
        Err(_) => "".to_string(),
    };
    writeln!(handle, "token {:?}", token).unwrap();


    // let response = connection.send_login()?;  // handle error with match
    // let status_code = response.status();
    // writeln!(handle, "status code: {:?}", status_code).unwrap();
    // let body: LoginResponse = match response.json() {
    //     Ok(body) => serde_json::from_value(body).unwrap(),
    //     Err(e) => panic!("Error while getting body of login response: {:?}", e)
    // };
    // writeln!(handle, "body: {:?}", body).unwrap();
    Ok(())
}
