use askama::Template;

#[derive(Template, Default, Clone, Copy)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub message: Option<&'a MessageKind<'a>>,
}

pub enum MessageKind<'a> {
    Link(Message<'a>),
    Error(Message<'a>),
}

pub struct Message<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
