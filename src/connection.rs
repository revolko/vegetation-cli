pub struct Connection {
    pub client: reqwest::blocking::Client,
}

impl Connection {
     pub fn build_connection() -> Connection {
         Connection {
             client: reqwest::blocking::Client::new()
         }
     }
}
