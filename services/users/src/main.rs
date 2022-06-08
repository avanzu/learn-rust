use actix_web::middleware::Logger;
use common::util;
use diesel::{r2d2::ConnectionManager, PgConnection};
use env_logger::Env;
use users::{handlers, Pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web};

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_dsn = util::database_url(String::from("postgres://localhost:5432/frugal-rose"));
    let manager = ConnectionManager::<PgConnection>::new(database_dsn);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Unable to create connection pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(handlers::configure)
    })
    .bind(util::bind_addr("PORT", 8080))?
    .run()
    .await
}
