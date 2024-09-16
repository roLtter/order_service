use axum::{
    routing::get,
    Json, Router, response::Html,
    extract::Extension,

};
use askama::Template;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;


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

async fn get_order(Extension(order_state): Extension<Arc<RwLock<Order>>>) -> Html<String> {
    let order = order_state.read().await.clone();
    let template = OrderTemplate { order };
    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string()))
}

#[tokio::main]
async fn main() {
    let order = Order {
        order_uid: "b563feb7b2b84b6test".to_string(),
        track_number: "WBILMTESTTRACK".to_string(),
        entry: "WBIL".to_string(),
        delivery: Delivery {
            name: "Test Testov".to_string(),
            phone: "+9720000000".to_string(),
            zip: "2639809".to_string(),
            city: "Kiryat Mozkin".to_string(),
            address: "Ploshad Mira 15".to_string(),
            region: "Kraiot".to_string(),
            email: "test@gmail.com".to_string(),
        },
        payment: Payment {
            transaction: "b563feb7b2b84b6test".to_string(),
            request_id: "".to_string(),
            currency: "USD".to_string(),
            provider: "wbpay".to_string(),
            amount: 1817,
            payment_dt: 1637907727,
            bank: "alpha".to_string(),
            delivery_cost: 1500,
            goods_total: 317,
            custom_fee: 0,
        },
        items: vec![Item {
            chrt_id: 9934930,
            track_number: "WBILMTESTTRACK".to_string(),
            price: 453,
            rid: "ab4219087a764ae0btest".to_string(),
            name: "Mascaras".to_string(),
            sale: 30,
            size: "0".to_string(),
            total_price: 317,
            nm_id: 2389212,
            brand: "Vivienne Sabo".to_string(),
            status: 202,
        }],
        locale: "en".to_string(),
        internal_signature: "".to_string(),
        customer_id: "test".to_string(),
        delivery_service: "meest".to_string(),
        shardkey: "9".to_string(),
        sm_id: 99,
        date_created: "2021-11-26T06:22:19Z".to_string(),
        oof_shard: "1".to_string(),
    };

    let app = Router::new()
        .route("/order", get(get_order))
        .layer(Extension(Arc::new(RwLock::new(order))));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
