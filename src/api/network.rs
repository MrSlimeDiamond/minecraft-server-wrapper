use std::collections::HashMap;
use config::Config;
use rocket::{self, serde::{Serialize, Deserialize, json::{Json, serde_json::json}}, log::LogLevel, request::{FromRequest, Outcome, Request}, http::Status};
use crate::send_cmd;


struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == Config::builder()
            .add_source(config::File::with_name("wrapper_config"))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap().get("api-key").unwrap()
        }

        match req.headers().get_one("key") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

#[derive(FromForm, Deserialize)]
struct SendcmdJson {
    command: String
}

#[derive(Serialize)]
struct OkJson {
    status: u16
}

#[post("/sendcmd", format = "application/json", data = "<body>")]
fn sendcommand(body: Json<SendcmdJson>, _k: ApiKey) -> Json<OkJson> {
    send_cmd!("{}", body.command);
    Json(OkJson { status: 200 })
}

#[launch]
pub fn start() -> _ {
    let config_file = Config::builder()
        .add_source(config::File::with_name("wrapper_config"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let mut config = rocket::Config::default();
    config.log_level = LogLevel::Critical;
    config.port = config_file.get("api-port").unwrap().parse().unwrap();

    rocket::custom(config).mount("/", routes![sendcommand])
}
