use crate::config::HttpSettings;
use actix_web::{error, get, http::{
    header::{self, ContentType},
    Method, StatusCode,
}, middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result, http};
use std::net;
use std::os::unix::net::SocketAddr;
use serde_json::json;

struct BridgeHttpServer {
    settings: HttpSettings,
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => Ok(Either::Left(HttpResponse::Forbidden().finish())),
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

async fn site_map() -> HttpResponse{
    HttpResponse::Ok()
    // .append_header((http::header::CONTENT_TYPE, "application/json"))
        .json(json!({
                "self": "/",
                "Healthcheck": "/healthcheck",
                "Bridge Status": "/stats"
    }))
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

impl BridgeHttpServer {
    pub async fn new(settings: HttpSettings) -> std::io::Result<()> {
        log::info!(
            "starting HTTP server at {}:{}",
            settings.address,
            settings.port
        );

        // let server: SocketAddr = settings.address
        //     .parse()
        //     .expect("Unable to parse socket address");

        HttpServer::new(|| {
            App::new()
                // .servce()
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_server;
    use futures::task::Spawn;
    use std::any::TypeId;
    use std::hash::Hasher;
    use actix_web::body::to_bytes;
    use futures::TryFutureExt;

    #[tokio::test]
    async fn test_default() {
        let s = BridgeHttpServer {
            settings: HttpSettings::default(),
        };
    }

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

