use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::{query, query_as};
use tide::http::StatusCode;
use tide::Request;
use tide::Response;
use tide::Server;
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() {
    let app = server().await;
    app.listen("127.0.0.1:8080").await.unwrap();
}

#[cfg(not(test))]
async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    Pool::new(&db_url).await.unwrap()
}

#[cfg(test)]
async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    Pool::new(&db_url).await.unwrap()
}

pub async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_pool = make_db_pool().await;
    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/users")
        .get(|req: Request<State>| async move {
            let db_pool: &PgPool = &req.state().db_pool;
            let users = query_as!(User, "select id, username from users")
                .fetch_all(db_pool)
                .await?;
            Ok(Response::new(StatusCode::Ok).body_json(&users)?)
        })
        .post(|mut req: Request<State>| async move {
            dbg!(&req);
            let db_pool = &req.state().db_pool.clone();
            let create_user = req.body_json::<CreateUser>().await?;
            dbg!(&db_pool);
            query!(
                r#"
                    insert into users (id, username)
                    values($1, $2)
                "#,
                Uuid::new_v4(),
                create_user.username
            )
            .execute(&*db_pool)
            .await?;
            Ok(Response::new(StatusCode::Created).body_json(&json!([]))?)
        });

    app
}

#[derive(Debug)]
pub struct State {
    db_pool: PgPool,
}

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    id: Uuid,
    username: String,
}
