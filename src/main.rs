use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::{Error, Result};
use libsql::{params, Builder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Todo: Check AUTHEY, LIBSQL_URL, LIBSQL_TOKEN is in .env.

    if env::var("LIBSQL_URL").is_err() {
        panic!("LIBSQL_URL needed to be set in environment");
    }

    if env::var("LIBSQL_TOKEN").is_err() {
        panic!("LIBSQL_TOKEN needed to be set in environment");
    }

    if env::var("AUTHKEY").is_err() {
        panic!("AUTHKEY needed to be set in environment");
    }

    HttpServer::new(|| App::new().service(root).service(receive))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::NotFound().body("nothing here")
}

#[post("/{tail:.*}")]
async fn receive(
    req: HttpRequest,
    path: web::Path<String>,
    req_body: String,
) -> impl Responder {
    let sender = match req.headers().get("X-Forwarded-For") {
        Some(ip) => ip.to_str().unwrap().to_string(),
        None => match req.peer_addr() {
            Some(val) => val.ip().to_string(),
            None => "".to_string(),
        },
    };

    let mut key = "/".to_string();
    key.push_str(path.as_str());

    // Todo: Validate req_body - proper json.
    let val = req_body;

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
    match record(sender, key, val).await {
        Ok(_) => HttpResponse::Ok().body("Thank you. Come again!\n"),
        Err(e) => {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().body("Something is wrong.\n")
        }
    }
}

async fn validate_json(string_json: String) -> Result<()> {
    Ok(())
}

async fn check_auth() -> Result<()> {
    Ok(())
}

async fn record(sender: String, key: String, val: String) -> Result<()> {
    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await?;
    let conn = db.connect().unwrap();

    let mut stmt = conn
        .prepare(
            "INSERT INTO message (sender, key, value) VALUES (?1, ?2, ?3)",
        )
        .await?;
    stmt.execute([sender, key, val]).await?;

    Ok(())
}
