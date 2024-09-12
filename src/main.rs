use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::{Error, Result};
use dotenv::dotenv;
use libsql::{params, Builder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Todo: Check AUTHEY, LIBSQL_URL, LIBSQL_TOKEN is in .env.
    env::var("LIBSQL_URL")
        .unwrap_or(panic!("LIBSQL_URL needed in envorinment or .env file"));
    env::var("LIBSQL_TOKEN")
        .unwrap_or(panic!("LIBSQL_TOKEN needed in environment or .env file"));
    env::var("LIBSQL_URL")
        .unwrap_or(panic!("AUTHKEY needed in environment or .env file"));

    HttpServer::new(|| App::new().service(root).service(receive))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::NotFound().body("nothing here")
}

#[post("/")]
async fn receive(req: HttpRequest, req_body: String) -> impl Responder {
    // Todo: Validate req_body - proper json.

    // Todo: Check Auth http header
    match req.headers().get("AUTHKEY") {
        Some(k) => {
            if k.to_str().unwrap().to_string() != env::var("AUTHKEY").unwrap()
            {
                return HttpResponse::Unauthorized().body("Not Allowed");
                ()
            }
        }
        _ => return HttpResponse::Unauthorized().body("Not Allowed"),
    }

    // Todo: Store into turso sql
    match record(req_body).await {
        Ok(_) => HttpResponse::Ok().body("Thank you"),
        Err(e) => {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().body("Something is wrong.")
        }
    }
}

async fn validate_json(string_json: String) -> Result<()> {
    Ok(())
}

async fn check_auth() -> Result<()> {
    Ok(())
}

async fn record(val: String) -> Result<()> {
    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await?;
    let conn = db.connect().unwrap();

    let mut stmt = conn
        .prepare("INSERT INTO message (content) VALUES (?1)")
        .await?;
    stmt.execute([val]).await?;

    Ok(())
}

/*
CREATE TABLE message (
  content TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
)
*/
