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

struct AppState {
    redis_url: String,
    order_id: Mutex<i32>,
}

/// Health endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// POST /order - Add new order (concurrency-safe, Redis-backed)
async fn add_order(state: web::Data<AppState>, order: web::Json<Order>) -> impl Responder {
    let mut id_guard = state.order_id.lock().unwrap();
    let id = *id_guard;
    *id_guard += 1;
    drop(id_guard);
    let mut new_order = order.into_inner();
