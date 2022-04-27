use crate::cfg::Cfg;
use crate::output;
use crate::output::{Output, OutputDriver};
use actix::{Actor, StreamHandler};
use actix_web::web::Buf;
use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::io::Read;
use std::sync::Arc;

/// Define HTTP actor
struct LogaggWs {
    outputs: Vec<Box<dyn Output>>,
}

impl LogaggWs {
    pub fn new(outputs: Vec<Box<dyn Output>>) -> Self {
        LogaggWs { outputs }
    }
}
impl Actor for LogaggWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LogaggWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                for op in self.outputs.iter_mut() {
                    match op.write(text.as_bytes()) {
                        Ok(_) => (),
                        Err(err) => {
                            log::warn!("failed to write text to output {}: {}", op.id(), err);
                        }
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                let mut dec = GzDecoder::new(bin.reader());
                let mut buffer = Vec::new();
                if let Err(err) = dec.read_to_end(&mut buffer) {
                    log::error!("failed to decode message: {}", err);
                    return;
                }

                for op in self.outputs.iter_mut() {
                    match op.write(&buffer) {
                        Ok(_) => (),
                        Err(err) => {
                            log::warn!("failed to write text to output {}: {}", op.id(), err);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

struct Drivers {
    drivers: Vec<Box<dyn OutputDriver>>,
}

async fn log_handle(
    req: HttpRequest,
    path: web::Path<(String,)>,
    data: web::Data<Arc<Drivers>>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut outputs = vec![];

    for driver in &data.drivers {
        match driver.open(&path.0).await {
            Ok(output) => {
                outputs.push(output);
            }
            Err(err) => {
                log::error!("failed to open output driver {}: {}", driver.id(), err);
                return Err(error::ErrorInternalServerError(err));
            }
        };
    }

    ws::start(LogaggWs::new(outputs), &req, stream)
}

pub async fn server(cfg: Cfg) -> Result<()> {
    let addr = cfg.listen.clone();
    let mut drivers = vec![];
    for out in cfg.output.iter() {
        let driver =
            output::driver(&out.kind, &out.config).context("failed to initialize output driver")?;
        drivers.push(driver);
    }

    let drivers = Arc::new(Drivers { drivers });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&drivers)))
            .route("/logs/{name}", web::get().to(log_handle))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
