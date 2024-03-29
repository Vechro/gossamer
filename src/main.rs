use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use askama::Template;
use gossamer::{actions, configuration::*, error::*, message::*};
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
    let target_url = Url::parse(&form.link).map_err(Error::ParseError)?;

    if !is_accepted_uri(target_url.scheme()) {
        Err(Error::InvalidScheme)?
    }

    let host_str = target_url.host_str().ok_or(Error::InvalidLink)?;

    // Why would we ever want a short link to another short link?
    if host_str == *VANITY_DOMAIN {
        Err(Error::InvalidLink)?
    }

    let key = web::block(move || {
        actions::insert_link(&DATABASE, target_url.as_str()).map_err(Error::DatabaseError)
    })
    .await??;
    let short_path = HASHER.encode(&[key]);

    let index_template = Index::new(Some(&MessageKind::Link(Message {
        title: "Here's your short link!",
        body: &format!("https://{}/{}", &*VANITY_DOMAIN, short_path),
    })))
    .render()
    .map_err(Error::TemplateError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(index_template))
}

#[get("/{short_path}")]
async fn redirect(short_path: web::Path<String>) -> Result<impl Responder> {
    let short_path = short_path.into_inner();
    let decoded = HASHER.decode(&short_path).map_err(Error::HasherError)?;

    let link =
        web::block(move || actions::get_link_by_key(&DATABASE, decoded[0]).ok_or(Error::NotFound))
            .await??;

    Ok(web::Redirect::to(link))
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Starting server at http://{}/", &*ADDRESS);

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(create_short_link)
            .service(redirect)
    })
    .bind(&*ADDRESS)?
    .run()
    .await
}
