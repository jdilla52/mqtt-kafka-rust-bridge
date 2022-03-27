use crate::bridge::BridgeStats;
use crate::config::HttpSettings;
use actix_web::web::Data;
use actix_web::{error, get, http, http::{
    header::{self, ContentType},
    Method, StatusCode,
}, middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result, rt};
use serde_json::json;
use std::{net, thread};
use std::os::unix::net::SocketAddr;
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
    let mut my_data = data.lock().await;
    HttpResponse::Ok()
        // .append_header((http::header::CONTENT_TYPE, "application/json"))
        .json(my_data.as_json())
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


async fn run_api(settings: HttpSettings,
                 bridge_stats: Arc<Mutex<BridgeStats>>, ) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(bridge_stats.clone()))
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

pub(crate) fn spawn_api(
    settings: HttpSettings,
    bridge_stats: Arc<Mutex<BridgeStats>>,
){
    tokio::spawn(async move {
        let server_future = run_api(settings.clone(), bridge_stats.clone());
        rt::System::new().block_on(server_future)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_server;
    use actix_web::body::to_bytes;
    use futures::task::Spawn;
    use futures::TryFutureExt;
    use std::any::TypeId;
    use std::hash::Hasher;

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
