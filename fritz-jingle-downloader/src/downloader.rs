use std::{io::{self, Read}, path::Path, path::PathBuf};

use indicatif::{ProgressBar, ProgressStyle};
use select::{document::Document, node::Node, predicate::{Attr, Class, Name, Predicate}};
use eyre::{eyre, Result};
use reqwest::{self, Client, Url, header};
use fritz_jingle_db::jingle::Jingle;
use tokio::{fs, io::AsyncWriteExt, io::copy};
use utils::math::map_range;

struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

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

    pub async fn run(&self) -> Result<()>{
        self.get_jingles_list().await?;

        Ok(())
    }

    async fn get_jingles_list(&self) -> Result<()> {
        let jingles_list = self.jingles_page.find(
            Attr("id", "main")
            .descendant(Class("last")
            .descendant(Name("article")))
        );

        let mut count: i32 = 0;
        let mut jingles: Vec<Jingle> = Vec::new();

        for node in jingles_list {
            // dbg!(node);
            if let Some(jingle) = self.generate_jingle_from_node(node) {
                if count < 5 {
                    jingles.push(jingle);
                }
            } else {
                continue;
            }

            count += 1;
        }

        for jingle in jingles {
            Downloader::download(jingle.url.as_str(), Path::new(".").to_path_buf()).await?;
        }

        Ok(())
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
    
    // This below is taken from https://www.reddit.com/r/rust/comments/9lrpru/download_file_with_progress_bar/
    // and from https://github.com/benkay86/async-applied
    // and adjusted to my needs
    async fn download(url: &str, to_dir: PathBuf) -> Result<PathBuf> {
        let url = Url::parse(url)?;
        let client = Client::new();
    
        let total_size = {
            let resp = client.head(url.as_str()).send().await?;
            if resp.status().is_success() {
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
                )).into());
            }
        };
    
        let mut request = client.get(url.as_str());
        let progress_bar_length: i32 = 40;
        let progress_bar = ProgressBar::new(total_size);
        progress_bar.set_style(ProgressStyle::default_bar()
                     .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                     .progress_chars("#>-"));
    
        let filename = url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("tmp.part");

        let file = to_dir.join(filename);
        let file_path = Path::new(&file);
    
        if file_path.exists() {
            let size = file_path.metadata()?.len() - 1;
            request = request.header(header::RANGE, format!("bytes={}-", size));
            progress_bar.inc(size);
        }
    
        // let mut source = DownloadProgress {
        //     progress_bar: pb,
        //     inner: request.send().await?,
        // };
    
        // let mut dest = fs::OpenOptions::new()
        //     .create(true)
        //     .append(true)
        //     .open(&file)
        //     .await?;
    
        let mut outfile = tokio::fs::File::create(&file_path).await?;
        let mut download = request.send().await?;

        while let Some(chunk) = download.chunk().await? {
            // let chunk_partsize = total_size / chunk.len() as u64;
            let delta = map_range(chunk.len(), 1, total_size as usize, 1, progress_bar_length as usize);
            println!("file: {} | total size: {} | chunk size: {} | delta: {}", &filename, total_size, chunk.len(), delta);

            progress_bar.inc(delta as u64);
            outfile.write(&chunk).await?;
        }

        // let _ = copy(&mut source, &mut dest).await?;
    
        println!(
            "Download of '{}' has been completed.",
            filename
        );
    
        Ok(file_path.to_path_buf())
    }
}