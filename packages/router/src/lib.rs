//! Wabper router
mod logger;
use logger::Logger;

use axum::{AddExtensionLayer, Router};
#[macro_use]
extern crate tracing;
use tower_http::trace::TraceLayer;
use wabper_common::Error;
use wabper_db::db_get_connection;

mod routes;

pub fn get_axum_router() -> Result<Router, Error> {
    let db_client = db_get_connection()?;

    Ok(Router::new()
        .nest("/api", routes::api::get_api_router())
        .layer(AddExtensionLayer::new(db_client))
        .layer(
            TraceLayer::new_for_http()
                .on_request(Logger)
                .on_response(Logger),
        ))
}
