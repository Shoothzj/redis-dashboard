extern crate core;

use std::env;

use actix_web::{App, HttpServer, web};

mod constant;
mod redis;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let log_level = match env::var("LOG_LEVEL") {
        Ok(level) => {
            if level == "trace" {
                log::LevelFilter::Trace
            } else if level == "debug" {
                log::LevelFilter::Debug
            } else if level == "info" {
                log::LevelFilter::Info
            } else if level == "warn" {
                log::LevelFilter::Warn
            } else if level == "error" {
                log::LevelFilter::Error
            } else {
                log::LevelFilter::Info
            }
        }
        Err(..) => {
            log::LevelFilter::Info
        }
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply().unwrap();
    HttpServer::new(|| {
        App::new().configure(config)
    })
        .bind(("0.0.0.0", 10004))?
        .run()
        .await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/redis").configure(redis::redis_router));
}
