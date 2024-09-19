use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::{Error, Result};
use libsql::{params, Builder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Todo: Check AUTHEY, LIBSQL_URL, LIBSQL_TOKEN is in .env.

    if env::var("LIBSQL_URL").is_err() {
        panic!("LIBSQL_URL needed to be set in environment");
    }

    if env::var("LIBSQL_TOKEN").is_err() {
        panic!("LIBSQL_TOKEN needed to be set in environment");
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
    let who = match req.headers().get("AUTHKEY") {
        Some(k) => {
            let a = check_auth(k.to_str().unwrap().to_string()).await.unwrap();
            if a.is_none() {
                return HttpResponse::Unauthorized().body("Not Allowed");
            } else {
                a.unwrap()
            }
        }
        _ => return HttpResponse::Unauthorized().body("Not Allowed"),
    };

    // Todo: Store into turso sql
    match record(sender, who, key, val).await {
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

async fn check_auth(auth_key_string: String) -> Result<Option<String>> {
    // Read auth.json

    let path = "auth.yaml";
    let data = fs::read_to_string(path)
        .expect(format!("Unable to read {}", path).as_str());
    //let res: Vec<HashMap<String, String>> = serde_yaml::from_str(&data)?;

    let parsed: HashMap<String, String> = serde_yaml::from_str(&data)?;

    for key in parsed.into_iter() {
        if key.1 == auth_key_string {
            return Ok(Some(key.0));
        }
    }

    Ok(None)
}

async fn record(
    sender: String,
    who: String,
    key: String,
    val: String,
) -> Result<()> {
    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await?;
    let conn = db.connect().unwrap();

    let mut stmt = conn
        .prepare(
            "INSERT INTO message (sender, who, key, value) VALUES (?1, ?2, ?3, ?4)",
        )
        .await?;
    stmt.execute([sender, who, key, val]).await?;

    Ok(())
}
