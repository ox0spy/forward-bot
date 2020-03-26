use std::env;
use serde::{Serialize, Deserialize};
use actix_web::{web, App, Result, HttpServer, HttpResponse};

use telegram_bot::requests::SendMessage;
use telegram_bot::types::ChatId;
use telegram_bot::{Api, Error};

#[derive(Debug, Deserialize)]
struct Payload {
    pub token: String,
    pub chat_id: i64,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    text: String,
}

async fn forward_message(payload: web::Json<Payload>) -> Result<HttpResponse> {
    let mut text = payload.text.clone();
    if let Err(e) = send_telegram_message(&payload.token, payload.chat_id, &payload.text).await {
        text = format!("{}", e);
    }

    Ok(HttpResponse::Ok().json(ResponseData {
        text,
    }))
}

#[actix_rt::main]
async fn main() {
    let bind_addr = env::var("BIND_ADDRESS").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/forward-telegram-text", web::post().to(forward_message))
    })
    .bind(bind_addr).unwrap()
    .run()
    .await.unwrap();
}

async fn send_telegram_message(token: &str, chat_id: i64, text: &str) -> Result<(), Error> {
    let chat_id = ChatId::new(chat_id);
    let api = Api::new(token);
    let msg = SendMessage::new(chat_id, text);
    api.send(msg).await?;

    Ok(())
}
