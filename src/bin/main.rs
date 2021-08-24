use actix_web::{
    error::BlockingError, get, http::header, middleware, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder, Result,
};
use askama::Template;
use keyva::{
    actions::{get_link_by_key, insert_link},
    error, is_accepted_uri, ADDRESS, DATABASE, HASHER, HOST,
};
use lazy_static::lazy_static;
use serde::Deserialize;
use url::Url;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

lazy_static! {
    static ref INDEX_TEMPLATE: String = Index.render().expect("Failed to render index template");
}

#[derive(Deserialize)]
pub struct FormData {
    link: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&*INDEX_TEMPLATE)
}

#[post("/")]
async fn create_short_link(form: web::Form<FormData>) -> Result<impl Responder, error::Error> {
    println!("Received: {}", &form.link);
    let target_url = Url::parse(&form.link)?;

    if !is_accepted_uri(target_url.scheme()) {
        Err(error::Error::InvalidLink)?
    }

    let host_str = target_url.host_str().ok_or(error::Error::InvalidLink)?;

    // Why would we ever want a short link to another short link?
    if host_str == &*HOST {
        Err(error::Error::InvalidLink)?
    }

    let key = insert_link(&DATABASE, target_url.as_str())?;
    let short_path = HASHER.encode(&[key]);

    println!("Link created: {}", short_path);

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(&*INDEX_TEMPLATE))
}

#[get("/{short_path}")]
async fn redirect(web::Path(short_path): web::Path<String>) -> Result<impl Responder> {
    let result = web::block(move || {
        let decoded = HASHER
            .decode(&short_path)
            .map_err(|_| BlockingError::Error(format!("Failed decoding: {}", short_path)))?;

        get_link_by_key(&DATABASE, decoded[0])
            .ok_or(BlockingError::Error("Failed resolving to link".to_string()))
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::NotFound().finish()
    });

    match result {
        Ok(link) => Ok(HttpResponse::Found()
            .set_header(header::LOCATION, link)
            .finish()),
        Err(r) => Ok(r),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Starting server at {}", &*ADDRESS);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(create_short_link)
            .service(redirect)
    })
    .bind(&*ADDRESS)?
    .run()
    .await
}
