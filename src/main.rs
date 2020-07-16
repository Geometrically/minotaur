mod routes;
mod database;
mod scopes;

use actix_web::{HttpServer, App, middleware};
use dotenv;
use routes::index::index;
use anyhow::{Result, Context};
use env_logger::Env;
use crate::database::mongo_database::connect;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let client = connect().await.context("Error while connecting to the database.");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new("\"%r\" %s %b %Dms"))
            .service(index)
    })
        .bind(dotenv::var("BIND_ADDR")?)?
        .run()
        .await?;
    Ok(())
}