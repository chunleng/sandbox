use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

use axum::{Json, Router, debug_handler, response::IntoResponse, routing::post};
use serde::Deserialize;
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(buy, check_item),
    components(schemas(Item))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_doc = ApiDoc::openapi();
    println!("{}", api_doc.to_json().unwrap());

    let app = Router::new()
        .route("/buy", post(buy))
        .route("/check_item", post(check_item));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
struct Item {
    id: usize,
    name: String,
}

static SHOPPING_CART: LazyLock<Mutex<HashMap<usize, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[utoipa::path(
    post,
    path = "/buy",
    request_body = Item,
    responses(
        (status = 200, description = "Items in the shopping cart")
    )
)]
#[debug_handler]
async fn buy(Json(item): Json<Item>) -> impl IntoResponse {
    let mut cart = SHOPPING_CART.lock().expect("lock error");
    let cart_item = cart.entry(item.id).or_insert(0);
    *cart_item = *cart_item + 1;
    format!("Added {} into the cart.", item.name).into_response()
}

#[utoipa::path(
    post,
    path = "/check_item",
    request_body = Item,
    responses(
        (status = 200, description = "Items in the shopping cart")
    )
)]
#[debug_handler]
async fn check_item(Json(item): Json<Item>) -> impl IntoResponse {
    let cart = SHOPPING_CART.lock().expect("lock error");
    let no_of_item = cart.get(&item.id).unwrap_or(&0);
    format!("In the cart, you have {}x of {}", no_of_item, item.name).into_response()
}
