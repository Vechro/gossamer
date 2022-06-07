use actix_files::Files;
use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, Responder, Result};
use askama::Template;
use gossamer::{
    actions, is_accepted_uri, message::*, prelude::*, ADDRESS, BLANK_INDEX_TEMPLATE, DATABASE,
    HASHER, VANITY_HOST,
};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct FormData {
    link: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(&**BLANK_INDEX_TEMPLATE)
}

#[post("/")]
async fn create_short_link(form: web::Form<FormData>) -> Result<impl Responder> {
    let target_url = Url::parse(&form.link).map_err(crate::Error::ParseError)?;

    if !is_accepted_uri(target_url.scheme()) {
        Err(crate::Error::InvalidScheme)?
    }

    let host_str = target_url.host_str().ok_or(crate::Error::InvalidLink)?;

    // Why would we ever want a short link to another short link?
    if host_str == &*VANITY_HOST {
        Err(crate::Error::InvalidLink)?
    }

    let key = web::block(move || {
        actions::insert_link(&DATABASE, target_url.as_str()).map_err(crate::Error::DatabaseError)
    })
    .await??;
    let short_path = HASHER.encode(&[key]);

    let index_template = Index::new(Some(&MessageKind::Link(Message {
        title: "Here's your short link!",
        body: &format!("https://{}/{}", &*VANITY_HOST, short_path),
    })))
    .render()
    .map_err(crate::Error::TemplateError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(index_template))
}

#[get("/{short_path}")]
async fn redirect(short_path: web::Path<String>) -> Result<impl Responder> {
    let short_path = short_path.into_inner();
    let decoded = HASHER.decode(&short_path).map_err(crate::Error::HasherError)?;

    let link = web::block(move || {
        actions::get_link_by_key(&DATABASE, decoded[0]).ok_or(crate::Error::NotFound)
    })
    .await??;

    Ok(HttpResponse::Found().insert_header((header::LOCATION, link)).finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Starting server at http://{}/", &*ADDRESS);

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(create_short_link)
            .service(redirect)
            .service(Files::new("/static", "./static"))
    })
    .bind(&*ADDRESS)?
    .run()
    .await
}
