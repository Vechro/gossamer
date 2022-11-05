use askama::Template;

use crate::configuration::{ASSETS_URL, VANITY_DOMAIN};

#[derive(Template, Clone, Copy)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub vanity_domain: &'a str,
    pub assets_url: &'a str,
    pub message: Option<&'a MessageKind<'a>>,
}

impl<'a> Index<'a> {
    pub fn new(message: Option<&'a MessageKind<'a>>) -> Self {
        Self { message, ..Default::default() }
    }
}

impl Default for Index<'_> {
    fn default() -> Self {
        Self { vanity_domain: &VANITY_DOMAIN, assets_url: &ASSETS_URL, message: Default::default() }
    }
}

pub enum MessageKind<'a> {
    Link(Message<'a>),
    Error(Message<'a>),
}

pub struct Message<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
