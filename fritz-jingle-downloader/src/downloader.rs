use std::{
    io::{self, Read},
    path::Path,
    path::PathBuf,
    sync::Arc,
};

use eyre::{eyre, Result};
use fritz_jingle_db::{jingle::Jingle, JinglesDb};
use futures::{stream, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{self, header, Client, Url};
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Class, Name, Predicate},
};
use tokio::{fs, io::copy, io::AsyncWriteExt, task::JoinHandle};

pub struct Downloader {
    jingles_page: Document,
    jingles_path: PathBuf
}

impl Downloader {
    pub async fn new(jingles_path: PathBuf) -> Result<Self> {
        let page_body = reqwest::get("https://www.fritz.de/programm/jingles/")
            .await?
            .text()
            .await?;

        let jingles_page = Document::from(page_body.as_str());
        // dbg!(&jingles_page);
        // println!("{}", page_body);
        Ok(Self { jingles_page, jingles_path })
    }

    pub async fn run(&self) -> Result<()> {
        let mut jingles: Vec<Jingle> = self.get_jingles_list().await?;
        self.download_jingles(&mut jingles).await?;
        self.write_db(&jingles).await?;

        dbg!(&jingles);
        Ok(())
    }

    async fn get_jingles_list(&self) -> Result<Vec<Jingle>> {
        let jingles_list = self
            .jingles_page
            .find(Attr("id", "main").descendant(Class("last").descendant(Name("article"))));

        let mut count: i32 = 0;
        let mut jingles: Vec<Jingle> = Vec::new();

        for node in jingles_list {
            // dbg!(node);
            if let Some(jingle) = self.generate_jingle_from_node(node) {
                if count < 20 {
                    jingles.push(jingle);
                }
            } else {
                continue;
            }

            count += 1;
        }

        // for jingle in jingles {
        //     Downloader::download(jingle, Path::new(".").to_path_buf()).await?;
        // }

        Ok(jingles)
    }

    fn generate_jingle_from_node(&self, node: Node) -> Option<Jingle> {
        let name = node
            .find(Name("h3").descendant(Name("span")))
            .next()?
            .text();
        let url = node.find(Name("a")).next()?.attr("href")?.to_string();
        let date_time = node
            .find(Name("time"))
            .next()?
            .attr("datetime")?
            .to_string();

        let jingle = Jingle {
            name: name,
            url: url,
            date_time: date_time,
            file_path: "foo".to_string(),
        };

        // dbg!(&jingle);

        Some(jingle)
    }

    async fn write_db(&self, jingles: &Vec<Jingle>) -> Result<()> {
        let jingles_json = serde_json::to_string_pretty(jingles)?;
        fs::write(self.jingles_path.join("db.json"), jingles_json).await?;

        Ok(())
    } 

    // TODO: Create database file from that
    // TODO: Update mechanism
    async fn download_jingles(&self, jingles: &mut Vec<Jingle>) -> Result<()> {
        let dl_path = self.jingles_path.join("files");
        fs::create_dir_all(&dl_path).await?;
        let multibar = std::sync::Arc::new(indicatif::MultiProgress::new());

        let main_pb = std::sync::Arc::new(
            multibar
                .clone()
                .add(indicatif::ProgressBar::new(jingles.len() as u64)),
        );
        main_pb.set_style(
            indicatif::ProgressStyle::default_bar().template("{msg} [{bar:40.yellow}] {pos}/{len}"),
        );
        main_pb.set_message("Total progress: ");
        main_pb.tick();

        let stream = stream::iter(jingles);
        let tasks = stream
            .enumerate()
            .for_each_concurrent(Some(10), |(_i, jingle)| {
                let multibar = multibar.clone();
                let main_pb = main_pb.clone();
                let dl_path = dl_path.clone();
                async move {
                    let file_path = tokio::task::spawn(Downloader::download_task(
                        jingle.clone(),
                        multibar,
                        dl_path,
                    ))
                    .await;
                    main_pb.inc(1);
                    
                    if let Ok(path) = file_path.unwrap() {
                        jingle.file_path = String::from(path.to_string_lossy());
                    }
                }
            });

        let multibar = {
            let multibar = multibar.clone();
            tokio::task::spawn_blocking(move || multibar.join())
        };

        tasks.await;

        main_pb.finish_with_message("Done");

        multibar.await??;

        Ok(())
    }

    // This below is taken from https://www.reddit.com/r/rust/comments/9lrpru/download_file_with_progress_bar/
    // and from https://github.com/benkay86/async-applied
    // and adjusted to my needs
    async fn download_task(
        jingle: Jingle,
        multibar: Arc<MultiProgress>,
        to_dir: PathBuf,
    ) -> Result<PathBuf> {
        let url = Url::parse(jingle.url.as_str())?;
        let client = Client::new();

        let total_size = {
            let resp = client.head(url.as_str()).send().await?;
            if resp.status().is_success() {
                // dbg!(resp.headers().get(header::CONTENT_LENGTH));

                resp.headers()
                    .get(header::CONTENT_LENGTH)
                    .and_then(|ct_len| ct_len.to_str().ok())
                    .and_then(|ct_len| ct_len.parse().ok())
                    .unwrap_or(0)
            } else {
                return Err(eyre!(format!(
                    "Couldn't download URL: {}. Error: {:?}",
                    url,
                    resp.status(),
                ))
                .into());
            }
        };
        let filename = format!("{}.mp3", jingle.name);
        let mut request = client.get(url.as_str());
        let progress_bar = multibar.add(indicatif::ProgressBar::new(total_size));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} {msg}")
                .progress_chars("#>-"),
        );
        progress_bar.set_message(&filename.as_str());

        let file = to_dir.join(&filename);
        let file_path = Path::new(&file);

        if file_path.exists() {
            let size = file_path.metadata()?.len() - 1;
            request = request.header(header::RANGE, format!("bytes={}-", size));
            progress_bar.inc(size);
        }

        let mut outfile = tokio::fs::File::create(&file_path).await?;
        let mut download = request.send().await?;

        while let Some(chunk) = download.chunk().await? {
            progress_bar.inc(chunk.len() as u64);
            outfile.write(&chunk).await?;
        }

        // println!(
        //     "Download of '{}' has been completed.",
        //     filename
        // );

        progress_bar.finish_and_clear();
        Ok(file_path.to_path_buf())
    }
}
