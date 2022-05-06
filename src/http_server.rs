#![allow(dead_code)]

use crate::bridge_stats::BridgeStats;
use crate::config::HttpSettings;
use actix_web::web::Data;
use actix_web::{http::Method, rt, web, App, Either, HttpResponse, HttpServer, Responder, Result};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

struct BridgeHttpServer {
    settings: HttpSettings,
    bridge_stats: Arc<Mutex<BridgeStats>>,
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => Ok(Either::Left(HttpResponse::Forbidden().finish())),
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

async fn site_map() -> HttpResponse {
    HttpResponse::Ok()
        // .append_header((http::header::CONTENT_TYPE, "application/json"))
        .json(json!({
                    "self": "/",
                    "Healthcheck": "/healthcheck",
                    "Bridge Status": "/stats"
        }))
}

async fn bridge_stats(data: Data<Arc<Mutex<BridgeStats>>>) -> HttpResponse {
    let my_data = data.lock().await;
    HttpResponse::Ok()
        // .append_header((http::header::CONTENT_TYPE, "application/json"))
        .json(my_data.as_json())
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn run_api(settings: HttpSettings, stats: Arc<Mutex<BridgeStats>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(stats.clone()))
            .route("/status", web::get().to(bridge_stats))
            .route("/healthcheck", web::get().to(health_check))
            .route("/", web::get().to(health_check))
            // default
            .default_service(web::to(default_handler))
    })
    .bind(settings.address)?
    .workers(1)
    .run()
    .await
}

pub(crate) fn spawn_api(settings: &HttpSettings, bridge_stats: &Arc<Mutex<BridgeStats>>) {
    let l_settings = settings.clone();
    let l_bridge_stats = bridge_stats.clone();
    tokio::spawn(async move {
        let server_future = run_api(l_settings, l_bridge_stats);
        rt::System::new().block_on(server_future)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response: HttpResponse = health_check().await;
        assert!(response.status().is_success())
    }

    #[tokio::test]
    async fn site_map_check_succeeds() {
        let response = site_map().await;
        assert!(response.status().is_success());
    }
}
