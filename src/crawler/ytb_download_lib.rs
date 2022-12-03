use crate::utils::error::Result;
use std::io::prelude::*;
use std::fs::File;
use futures_util::StreamExt;


pub async fn download(url: String, v_id:String)->Result<String> {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;
    let file_size = resp.content_length().unwrap();
    let file_path = format!("/Users/fieldlee/tmp/{}.mp4",v_id);

    let mut file = File::create(file_path.clone()).unwrap();

    let mut stream = resp.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) =>{
                file.write_all(&bytes)
            .or(Err(format!("download Error while writing to file")))?;
            }
            Err(e)=> println!("download Error :{:?}",e),
        }
    }
    Ok(file_path)
}