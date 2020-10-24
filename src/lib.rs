
use reqwest::Error;
use reqwest::blocking::{Client, Response};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebGame {
    pub id: String,
    pub moves: Vec<String>,
    pub board: Vec<Vec<u8>>,
    pub render: String,
}

pub struct WebClient {
    client: Client,
    hostname: String,
}

impl WebClient {
    pub fn from_hostname(hostname: &str) -> WebClient {
        WebClient {
            client: Client::new(),
            hostname: hostname.to_string(),
        }
    }

    pub fn get_latest_web_game(&self, game_id: &str) -> Result<WebGame, Box<dyn std::error::Error>> {
        let request_url = format!("https://{}/api/games/{}", self.hostname, game_id);
        let response = reqwest::blocking::get(&request_url)?;
        let web_game: WebGame = response.json()?;
        Result::Ok(web_game)
    }

    pub fn submit_move(&self, web_game: &WebGame, next_move: &str) -> Result<Response, Error> {
        let move_number = web_game.moves.len();
        let request_url = format!("https://{}/api/games/{}/moves/{}", self.hostname, web_game.id, move_number);
        self.client.post(request_url.as_str()).json(next_move).send()
    }
}


