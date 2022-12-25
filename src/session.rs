use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager}
};
use crate::db;
use crate::models::NewConversation;
use crate::server;

const HEARTBEAT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct WsChatSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
    pub db_pool: web::Data<DbPool>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ChatType {
    TYPING,
    TEXT,
    CONNECT,
    DISCONNECT,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    pub chat_type: ChatType,
    pub value: Vec<String>,
    pub room_id: String,
    pub user_id: String,
    pub id: usize,
}