use serde::{Deserialize, Serialize};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use reqwest::blocking::{Client, Response};
use reqwest::Error;

use crate::cli_args::{RegisterArgs, LoginArgs, CreatePlantArgs};
use crate::config::{VegConfig, VEG_CONFIG_NAME};

fn create_auth_header() -> HeaderMap {
    let conf: VegConfig = confy::load(VEG_CONFIG_NAME, None).unwrap();
    let token = conf.token;

    let mut auth_header = String::from("Token ");
    auth_header.push_str(&token);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header.parse().unwrap());

    return headers;
}

pub struct Connection {
    pub client: Client,
    url: String,  // should change it to &str
}

impl Connection {
     pub fn build_connection() -> Connection {
         Connection {
             client: Client::new(),
             url: String::from("http://127.0.0.1:8080/api/v1/"),
         }
     }

     pub fn send_get(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, Error> {
         let headers = match headers {
             Some(h) => h,
             None => HeaderMap::new(),
         };

         return self.client.get(self.url.to_owned() + endpoint).headers(headers).send();
     }

     pub fn send_post<T: Serialize>(&self, endpoint: &str, body: T, headers: Option<HeaderMap>) -> Result<Response, Error> {
         let headers = match headers {
             Some(h) => h,
             None => HeaderMap::new(),
         };

         return self.client.post(self.url.to_owned() + endpoint)
             .json(&body).headers(headers).send();
     }

     fn send_delete(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, Error> {
         let headers = match headers {
             Some(h) => h,
             None => HeaderMap::new(),
         };

         return self.client.delete(self.url.to_owned() + endpoint)
             .headers(headers).send();
     }

     pub fn send_register(&self, args: RegisterArgs) -> Result<Response, Error> {
         let register_body = RegisterBody {
             email: String::from(args.email),
             username: String::from(args.username),
             password: String::from(args.password),
         };
         self.send_post("register", register_body, None)
     }

     pub fn send_login(&self, args: LoginArgs) -> Result<Response, Error> {
         let login_body = LoginBody {
             username: String::from(args.username),
             password: String::from(args.password),
         };
         self.send_post("login", login_body, None)
     }

     pub fn send_list_plants(&self) -> Result<Response, Error> {
         let headers = create_auth_header();
         return self.send_get("plants", Some(headers));
     }

     pub fn send_create_plant(&self, create_args: CreatePlantArgs) -> Result<Response, Error> {
         let headers = create_auth_header();
         return self.send_post("plants", create_args, Some(headers));
     }

     pub fn send_delete_plant(&self, plant_id: u32) -> Result<Response, Error> {
         let headers = create_auth_header();
         return self.send_delete(&format!("plants/{}", plant_id), Some(headers));
     }
}


#[derive(Debug, Serialize, Deserialize)]
struct RegisterBody {
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterLoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantResponse {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub last_watered: String,
    pub water_frequency_summer: u32,
    pub water_frequency_winter: u32,
    pub owner: u32,
    pub watering_type: String,
    pub drought_tolerance: String,
    pub light_requirement: String,
}
