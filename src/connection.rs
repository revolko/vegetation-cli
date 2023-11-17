use serde::{Deserialize, Serialize};

pub struct Connection {
    pub client: reqwest::blocking::Client,
    url: String,  // should change it to &str
}

impl Connection {
     pub fn build_connection() -> Connection {
         Connection {
             client: reqwest::blocking::Client::new(),
             url: String::from("http://127.0.0.1:8080/api/v1/"),
         }
     }

     pub fn send_get(&self, endpoint: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
         self.client.get(self.url.to_owned() + endpoint).send()
     }

     pub fn send_post<T: Serialize>(&self, endpoint: &str, body: T) -> Result<reqwest::blocking::Response, reqwest::Error> {
         self.client.post(self.url.to_owned() + endpoint)
             .json(&body)
             .send()
     }

     pub fn send_register(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
         let register_body = RegisterBody {
             email: String::from("random@mail.com"),
             username: String::from("revolko"),
             password: String::from("my_pass_1234"),
         };
         self.send_post("register", register_body)
     }

     pub fn send_login(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
         let login_body = LoginBody {
             username: String::from("revolko"),
             password: String::from("my_pass_1234"),
         };
         self.send_post("login", login_body)
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
pub struct LoginResponse {
    token: String,
}
