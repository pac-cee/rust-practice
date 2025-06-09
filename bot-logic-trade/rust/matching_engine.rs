// Rust Matching Engine Microservice (Actix-web + Redis)
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Order {
    id: i32,
    user_id: String,
    order_type: String, // "buy" or "sell"
    price: f64,
    amount: f64,
    remaining: f64,
    status: String, // "open", "matched", "cancelled"
}

