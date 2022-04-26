use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::cfg::Cfg;

/// Define HTTP actor
struct LogaggWs;

impl Actor for LogaggWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LogaggWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn log_handle(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    data: web::Data<Cfg>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(LogaggWs {}, &req, stream);
    resp
}

pub async fn server(cfg: Cfg) -> std::io::Result<()> {
    let addr = cfg.listen.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(cfg.clone())
            .route("/logs/{contract}/{name}", web::get().to(log_handle))
    })
    .bind(addr)?
    .run()
    .await
}
