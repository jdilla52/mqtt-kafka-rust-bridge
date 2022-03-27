use crate::config::HttpSettings;
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use std::net;
use std::os::unix::net::SocketAddr;

struct BridgeHttpServer {
    settings: HttpSettings,
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => Ok(Either::Left(HttpResponse::Forbidden().finish())),
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
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
    use std::any::TypeId;

    #[tokio::test]
    async fn test_default() {
        let s = BridgeHttpServer {
            settings: HttpSettings::default(),
        };
    }
}
