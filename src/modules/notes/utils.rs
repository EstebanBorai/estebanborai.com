use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use yaml_front_matter::{Document, YamlFrontMatter};

#[derive(Clone, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub date: String,
}

pub fn parse_markdown(value: &str) -> (Metadata, String) {
    let document: Document<Metadata> = YamlFrontMatter::parse::<Metadata>(&value).unwrap();
    let mut opts = Options::empty();

    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&document.content, opts);
    let mut parsed = String::default();

    push_html(&mut parsed, parser);
    (document.metadata, parsed)
}

pub fn article_url_from_location(_id: String) -> String {
    String::from("https://raw.githubusercontent.com/EstebanBorai/EstebanBorai/main/notes/001-installing-the-rust-programming-language-on-windows.md")
}
