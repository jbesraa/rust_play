use serde_json::json;
use sqlx::PgPool;
use sqlx::Pool;
use tide::http::StatusCode;
use tide::Request;
use tide::Response;
use tide::Server;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() {
    let app = server().await;
    app.listen("127.0.0.1:8080").await.unwrap();
}

pub async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|_req: Request<State>| async move {
        let json = json!([1, 2, 3]);
        Ok(Response::new(StatusCode::Ok).body_json(&json)?)
    });
    app
}

#[derive(Debug)]
pub struct State {
    db_pool: PgPool,
}
