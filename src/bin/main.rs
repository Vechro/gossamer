use std::{env, fs::File, io::BufReader};

use actix_web::{
    error::BlockingError, get, http::header, middleware, post, web, App, HttpResponse, HttpServer,
    Responder, Result,
};
use askama::Template;
use gossamer::{actions, is_accepted_uri, message::*, prelude::*, ADDRESS, DATABASE, HASHER, HOST};
use lazy_static::lazy_static;
use rustls::{
    internal::pemfile::{certs, pkcs8_private_keys},
    NoClientAuth, ServerConfig,
};
use serde::Deserialize;
use url::Url;

lazy_static! {
    static ref BLANK_INDEX_TEMPLATE: String =
        Index::default().render().expect("Failed to render index template");
}

#[derive(Deserialize)]
pub struct FormData {
    link: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(&*BLANK_INDEX_TEMPLATE)
}

#[post("/")]
async fn create_short_link(form: web::Form<FormData>) -> Result<impl Responder, crate::Error> {
    // TODO: Make async
    let target_url = Url::parse(&form.link)?;

    if !is_accepted_uri(target_url.scheme()) {
        Err(crate::Error::InvalidLink)?
    }

    let host_str = target_url.host_str().ok_or(crate::Error::InvalidLink)?;

    // Why would we ever want a short link to another short link?
    if host_str == &*HOST {
        Err(crate::Error::InvalidLink)?
    }

    let key = actions::insert_link(&DATABASE, target_url.as_str())?;
    let short_path = HASHER.encode(&[key]);

    println!("Link created: {} => {}", short_path, &form.link);

    let index_template = Index {
        message: Some(&MessageKind::Error(Message {
            title: "Here's your short link!",
            body: &short_path,
        })),
    }
    .render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(index_template))
}

#[get("/{short_path}")]
async fn redirect(web::Path(short_path): web::Path<String>) -> Result<impl Responder> {
    let link = web::block(move || {
        let decoded = HASHER
            .decode(&short_path)
            .map_err(|e| BlockingError::Error(crate::Error::HasherError(e)))?;

        actions::get_link_by_key(&DATABASE, decoded[0])
            .ok_or(BlockingError::Error(crate::Error::NotFound))
    })
    .await?;

    Ok(HttpResponse::Found().set_header(header::LOCATION, link).finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let cert_path = env::var("TLS_CERT_PATH").expect("Couldn't find TLS_CERT_PATH");
    let key_path = env::var("TLS_KEY_PATH").expect("Couldn't find TLS_KEY_PATH");

    println!("Starting server at https://{}/", &*ADDRESS);

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file =
        &mut BufReader::new(File::open(cert_path).expect("Unable to read certificate file"));
    let key_file = &mut BufReader::new(File::open(key_path).expect("Unable to read key file"));
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(create_short_link)
            .service(redirect)
    })
    .bind_rustls(&*ADDRESS, config)?
    .run()
    .await
}
