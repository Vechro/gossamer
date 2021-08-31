use actix_files::Files;
use actix_web::{
    error::BlockingError, get, http::header, post, web, App, HttpResponse, HttpServer, Responder,
    Result,
};
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
    if host_str == &*VANITY_HOST {
        Err(crate::Error::InvalidLink)?
    }

    let key = actions::insert_link(&DATABASE, target_url.as_str())?;
    let short_path = HASHER.encode(&[key]);

    let index_template = Index::new(Some(&MessageKind::Link(Message {
        title: "Here's your short link!",
        body: &format!("https://{}/{}", &*VANITY_HOST, short_path),
    })))
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
