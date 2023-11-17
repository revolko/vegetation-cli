mod connection;

use clap::Parser;
use std::io::{Write, self};

use connection::Connection;

#[derive(Debug, Parser)]
struct CliParameters {
    operation: String,  // verbose option
}

fn main() -> Result<(), Box<reqwest::Error>> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let args = CliParameters::parse();
    writeln!(handle, "operation: {}", args.operation).unwrap();

    let connection = Connection::build_connection();

    let body = connection.client.get("http://127.0.0.1:8080/api/v1/plants").send()?;
    writeln!(handle, "body: {:?}", body).unwrap();
    Ok(())
}
