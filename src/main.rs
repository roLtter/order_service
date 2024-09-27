use axum::{
    routing::get,
    Router, response::Html,
    extract::Extension,
};
use askama::Template;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::fs;
use log::{info, error};
use clap::{Command, Arg};

#[derive(Template)]
#[template(path = "order.html")]
struct OrderTemplate {
    order: Order,
}

#[derive(Debug, Deserialize, Clone)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct Item {
    chrt_id: i32,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i32,
    brand: String,
    status: i32,
}

async fn load_order_from_json(path: &str) -> Result<Order, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path).await?;
    let order: Order = serde_json::from_str(&data)?;
    Ok(order)
}

async fn get_order(Extension(order_state): Extension<Arc<RwLock<Order>>>) -> Html<String> {
    let order = order_state.read().await.clone();
    let template = OrderTemplate { order };
    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string()))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = Command::new("Order Service")
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .default_value("3000")
            .help("Sets the port to listen on"))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .default_value("model.json")
            .help("Path to the order JSON file"))
        .get_matches();

    let port: u16 = matches.get_one::<String>("port").unwrap().parse().unwrap_or_else(|_| {
        error!("Invalid port number, using default port 3000");
        3000
    });
    let file_path = matches.get_one::<String>("file").unwrap();

    info!("Starting server on port: {}", port);
    info!("Loading order from file: {}", file_path);

    let order = match load_order_from_json(file_path).await {
        Ok(order) => order,
        Err(e) => {
            error!("Failed to load order from file: {}", e);
            return;
        }
    };

    let app = Router::new()
        .route("/order", get(get_order))
        .layer(Extension(Arc::new(RwLock::new(order))));

    if let Err(e) = axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
    {
        error!("Server failed: {}", e);
    } else {
        info!("Server started successfully on port {}", port);
    }
}
