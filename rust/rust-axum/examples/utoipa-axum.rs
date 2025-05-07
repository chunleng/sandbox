use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex},
};

use axum::{Json, debug_handler, response::IntoResponse, routing::get};
use serde::Deserialize;
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (app, api_doc) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(buy))
        .routes(routes!(check_item))
        .route("/hidden", get(hidden))
        .split_for_parts();
    println!("{}", api_doc.to_json().unwrap());
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

async fn hidden() -> impl IntoResponse {
    "I am hidden!".into_response()
}
