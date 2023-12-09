use serde::de::DeserializeOwned;
use std::env::{self};

use reqwest::blocking::Response;
use reqwest::Error;

use crate::cli_args::{RegisterArgs, LoginArgs, CreatePlantArgs};
use crate::connection::{Connection, RegisterLoginResponse, PlantResponse};

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
    };
    panic!("Request error");
}

fn get_response_body<T: DeserializeOwned>(response: Result<Response, Error>) -> T {
    let response = match response {
        Ok(res) => res,
        Err(e) => panic!("Request error {:?}", e),
    };

    if !response.status().is_success() {
        // Server responded with an error
        handle_server_error(response);
        std::process::exit(-1); // not very pretty but show compiler that response can be moved
    }

    return match response.json() {
        Ok(body) => match serde_json::from_value(body) {
            Ok(val) => val,
            Err(e) => panic!("Cannot parse request body to struct {:?}", e),
        },
        Err(e) => panic!("Cannot read json bdoy from the response {:?}", e),
    };
}


pub fn register(register_args: RegisterArgs, connection: Connection) -> () {
    let response = connection.send_register(register_args);

    let body: RegisterLoginResponse = get_response_body(response);
    set_token(body.token);
    println!("Successful registration and login. You can manage your biom now.");
}

pub fn login(login_args: LoginArgs, connection: Connection) -> () {
    let response = connection.send_login(login_args);

    let body: RegisterLoginResponse = get_response_body(response);
    set_token(body.token);
    println!("Successful login. You can manage your biom now.");
}

pub fn list_plants(connection: Connection) -> () {
    let response = connection.send_list_plants();

    let body: Vec<PlantResponse> = get_response_body(response);
    println!("Plants:\n{:?}", body);
}

pub fn create_plant(create_args: CreatePlantArgs, connection: Connection) -> () {
    let response = connection.send_create_plant(create_args);

    let body: PlantResponse = get_response_body(response);
    println!("Created plant:\n{:?}", body);
}
