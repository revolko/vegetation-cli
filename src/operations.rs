use serde::de::DeserializeOwned;

use reqwest::blocking::Response;
use reqwest::Error;

use crate::cli_args::{RegisterArgs, LoginArgs, CreatePlantArgs, DeletePlantArgs, SetServerArgs};
use crate::connection::{Connection, RegisterLoginResponse, PlantResponse};
use crate::config::{VegConfig, VEG_CONFIG_NAME};

fn set_token(token: String) -> () {
    let mut conf: VegConfig = match confy::load(VEG_CONFIG_NAME, None) {
        Ok(conf) => conf,
        Err(e) => panic!("Unable to retrieve the app configuration {:?}", e),
    };

    conf.token = token;
    confy::store(VEG_CONFIG_NAME, None, conf).unwrap();
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

pub fn delete_plant(delete_args: DeletePlantArgs, connection: Connection) -> () {
    let response = match connection.send_delete_plant(delete_args.plant_id) {
        Ok(res) => res,
        Err(e) => panic!("Request error {:?}", e),
    };

    if !response.status().is_success() {
        handle_server_error(response);
    };
}

pub fn set_server(set_args: SetServerArgs) -> () {
    let mut conf: VegConfig = match confy::load(VEG_CONFIG_NAME, None) {
        Ok(conf) => conf,
        Err(e) => panic!("Unable to retrieve app configuration {:?}", e),
    };

    conf.url = set_args.url;

    match confy::store(VEG_CONFIG_NAME, None, conf) {
        Ok(_) => (),
        Err(e) => panic!("Unable to store updated config {:?}", e),
    };
}
