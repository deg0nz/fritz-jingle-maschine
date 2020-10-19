use select::{document::Document, node::Node, predicate::{Attr, Class, Name, Predicate}};
use eyre::Result;
use reqwest;
use fritz_jingle_db::jingle::Jingle;

pub struct Downloader {
    jingles_page: Document
}

impl Downloader {
    pub async fn new() -> Result<Self> {
        let page_body = reqwest::get("https://www.fritz.de/programm/jingles/")
            .await?
            .text()
            .await?;
        
        let jingles_page = Document::from(page_body.as_str());
        // dbg!(&jingles_page);
        // println!("{}", page_body);
        Ok(Self {
            jingles_page
        })
    }

    pub fn run(&self) {
        self.get_jingles_list();
    }

    fn get_jingles_list(&self) {
        let jingles_list = self.jingles_page.find(
            Attr("id", "main")
            .descendant(Class("last")
            .descendant(Name("article")))
        );

        for node in jingles_list {
            // dbg!(node);
            let jingle = self.generate_jingle_from_node(node);
        }
    }

    fn generate_jingle_from_node(&self, node: Node) -> Option<Jingle> {
        let name = node.find(Name("h3").descendant(Name("span"))).next()?.text();
        let url = node.find(Name("a")).next()?.attr("href")?.to_string();
        let date_time = node.find(Name("time")).next()?.attr("datetime")?.to_string();
        
        let jingle = Jingle {
            name: name,
            url: url,
            date_time: date_time,
            file_path: "foo".to_string(),
        };

        dbg!(&jingle);

        Some(jingle)
    }
}