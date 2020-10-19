use select::{document::Document, predicate::{Attr, Class, Name, Predicate}};
use eyre::Result;
use reqwest;

pub struct Downloader {
    jingles_page: Document
}

impl Downloader {
    pub async fn new() -> Result<Self> {
        let page_body = reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?;
        
        let jingles_page = Document::from(page_body.as_str());

        Ok(Self {
            jingles_page
        })
    }

    pub fn run(&self) {
        self.get_jingles_list();
    }

    fn get_jingles_list(&self) {
        let jingles_list = self.jingles_page.find(Attr("id", "main").descendant(Class("last").descendant(Name("article"))));
        for node in jingles_list {
            dbg!(node);
        }
        
    }
}