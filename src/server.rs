use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use flate2::read::GzDecoder;
use std::io::Read;

use crate::cfg::Cfg;

/// Define HTTP actor
struct LogaggWs {
    pub data: web::Data<Cfg>,
    pub path: web::Path<(String, String)>,
}

impl Actor for LogaggWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LogaggWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

        let filename = format!("{}-{}", self.path.0, self.path.1);

        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                for op in &self.data.output {
                    let res = op.write(&filename, text.as_bytes());
                    if let Err(err) = res { log::warn!("{:?}", err) }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                let data: Result<Vec<_>, _> = bin.bytes().collect();
                let data = data.unwrap();
                let mut dec = GzDecoder::new(&data[..]);
                let mut buf = String::new();
                dec.read_to_string(&mut buf).unwrap();
                
                for op in &self.data.output {
                    let res = op.write(&filename, buf.as_bytes());
                    if let Err(err) = res { log::warn!("{:?}", err) }
                }
            }
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
    let resp = ws::start(LogaggWs { data, path }, &req, stream);
    resp
}

pub async fn server(cfg: Cfg) -> std::io::Result<()> {
    let addr = cfg.listen.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cfg.clone()))
            .route("/logs/{contract}/{name}", web::get().to(log_handle))
    })
    .bind(addr)?
    .run()
    .await
}
