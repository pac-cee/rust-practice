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
    new_order.id = id;
    new_order.remaining = new_order.amount;
    new_order.status = "open".to_string();
    let client = Client::open(state.redis_url.clone()).unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let order_key = format!("order:{}", new_order.id);
    let order_json = serde_json::to_string(&new_order).unwrap();
    let _: () = con.set(&order_key, order_json).await.unwrap();
    if new_order.order_type == "buy" {
        let _: () = con.zadd("buy_orders", new_order.id, -new_order.price).await.unwrap();
    } else {
        let _: () = con.zadd("sell_orders", new_order.id, new_order.price).await.unwrap();
    }
    match_orders(&mut con).await;
    HttpResponse::Ok().json(new_order)
}

