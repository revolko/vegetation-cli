use std::env::{self};

use reqwest::blocking::Response;

use crate::cli_args::{RegisterArgs, LoginArgs};
use crate::connection::{Connection, RegisterLoginResponse};

fn set_token(value: String) -> () {
    // SET TOKEN for the process only (when first login, or change of token)
    env::set_var("VEGETATION_TOKEN", &value); 
    match set_env::check_or_set("VEGETATION_TOKEN", value) {
        Ok(_) => (),
        Err(e) => panic!("Cannot set TOKEN env var: {:?}", e),
    };
}

fn handle_server_error(response: Response) {
    println!("Request error {:?}", response.status());
    if response.status().is_server_error() {
        println!("This really is not server's fault. Jokes aside, it is. :(");
    }
    match response.text() {
        Ok(text) => println!("Error: {:?}", text),
        Err(_) => panic!("Cannot parse response text"),
    }
    panic!("Request error");
}


pub fn register(register_args: RegisterArgs, connection: Connection) -> () {
    let response = match connection.send_register(register_args) {
        Ok(response) => response,
        Err(e) => panic!("Register error {:?}", e),
    };

    if !response.status().is_success() {
        // Server responded with an error --> to the Future add handling of different errors
        return handle_server_error(response);
    }

    let body: RegisterLoginResponse = match response.json() {
        Ok(body) => match serde_json::from_value(body) {
            Ok(val) => val,
            Err(e) => panic!("Cannot parse Register Body {:?}", e),
        }
        Err(e) => panic!("Cannot read json body from the response: {:?}", e),
    };

    set_token(body.token);
    println!("Successful registration and login. You can manage your biom now.");
}

pub fn login(login_args: LoginArgs, connection: Connection) -> () {
    let response = match connection.send_login(login_args) {
        Ok(response) => response,
        Err(e) => panic!("Login error {:?}", e),
    };

    if !response.status().is_success() {
        // Server responded with an error
        return handle_server_error(response);
    }

    let body: RegisterLoginResponse = match response.json() {
        Ok(body) => match serde_json::from_value(body) {
            Ok(val) => val,
            Err(e) => panic!("Cannot parse Login Body {:?}", e),
        },
        Err(e) => panic!("Cannot read json body from the response: {:?}", e),
    };

    set_token(body.token);
    println!("Successful login. You can manage your biom now.");
}
