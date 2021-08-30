use askama::Template;

use crate::VANITY_HOST;

#[derive(Template, Clone, Copy)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub vanity_host: &'a str,
    pub message: Option<&'a MessageKind<'a>>,
}

impl<'a> Index<'a> {
    pub fn new(message: Option<&'a MessageKind<'a>>) -> Self {
        Self { message, ..Default::default() }
    }
}

impl Default for Index<'_> {
    fn default() -> Self {
        Self { vanity_host: &VANITY_HOST, message: Default::default() }
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
