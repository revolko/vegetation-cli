mod connection;

use clap::{Parser, Subcommand};
use std::io::{Write, self};

use connection::{Connection, LoginResponse};

#[derive(Debug, Subcommand)]
enum CLIOperation {
    Register,
    Login,
    ListPlants,
}

#[derive(Debug, Parser)]
struct CliParameters {
    #[clap(subcommand)]
    operation: CLIOperation,  // verbose option
}

fn main() -> Result<(), Box<reqwest::Error>> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let args = CliParameters::parse();
    match args.operation {
        CLIOperation::ListPlants => writeln!(handle, "got list").unwrap(),
        CLIOperation::Login => writeln!(handle, "got login").unwrap(),
        CLIOperation::Register => writeln!(handle, "got Register").unwrap(),
        _ => writeln!(handle, "unkown").unwrap(),
    }

    let connection = Connection::build_connection();

    let response = connection.send_login()?;  // handle error with match
    let status_code = response.status();
    writeln!(handle, "status code: {:?}", status_code).unwrap();
    let body: LoginResponse = match response.json() {
        Ok(body) => serde_json::from_value(body).unwrap(),
        Err(e) => panic!("Error while getting body of login response: {:?}", e)
    };
    writeln!(handle, "body: {:?}", body).unwrap();
    Ok(())
}
