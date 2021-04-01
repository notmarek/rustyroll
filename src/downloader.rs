use std::process::Command;
use futures::{stream, StreamExt};
use libaes::Cipher;
use m3u8_rs::playlist::Playlist;
use m3u8_rs::playlist::{MasterPlaylist, MediaPlaylist};
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::prelude::*;
use tokio::sync::Mutex;
use std::io::{self};
use std::sync::Arc;

async fn parse_master(hls_uri: &str) -> Option<MasterPlaylist> {
    let hls: String = reqwest::get(hls_uri).await.unwrap().text().await.unwrap();
    let parsed = m3u8_rs::parse_playlist_res(&hls.as_bytes());
    match parsed {
        Ok(Playlist::MasterPlaylist(pl)) => Some(pl),
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
        _ => None,
    }
}
async fn parse_playlist(hls_uri: &str) -> Option<MediaPlaylist> {
    let hls: String = reqwest::get(hls_uri).await.unwrap().text().await.unwrap();
    let parsed = m3u8_rs::parse_playlist_res(&hls.as_bytes());
    match parsed {
        Ok(Playlist::MediaPlaylist(pl)) => Some(pl),
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
        _ => None,
    }
}
fn pop(bytes: &[u8]) -> &[u8; 16] {
    bytes.try_into().expect("slice with incorrect length") // not from stackoverflow 100%
}

struct SegDownloaded {
    part_number: u32,
    file: bytes::Bytes,
}

#[cfg(not(target_os = "windows"))]
async fn remux(out: &str, segments: u32) {
    let mut args = Vec::new();
    for i in 1..segments {
        args.push(format!("seg.{}.ts", i));
    }
    println!("Merging transport stream files.");
    let output = Command::new("cat").args(args).output().unwrap();
    println!("status: {}", output.status);
    let mut file = OpenOptions::new().write(true).create(true).open("full.ts").unwrap();
    file.write_all(&output.stdout).unwrap();
    println!("Merge done, fixing up the transport stream with ffmpeg.");
    Command::new("ffmpeg")
        .arg("-i")
        .arg("full.ts")
        .arg("-c")
        .arg("copy")
        .arg("full.final.ts")
        .output()
        .unwrap();
    println!("Fixing done, multiplexing resources into Matroska.");
    Command::new("mkvmerge").arg("@options.unix.json").arg("-o").arg(out).output().unwrap();
    println!("Finished your file is ready!")
}

#[cfg(target_os = "windows")]
async fn remux(out: &str, segments: u32) {
    let mut segment_string = "seg.1.ts".to_string();
    for i in 2..segments {
        segment_string = format!("{}+seg.{}.ts", segment_string, i);
    }
    println!("Merging transport stream files.");
    Command::new("cmd")
        .arg("/C")
        .arg("copy")
        .arg("/b")
        .arg(segment_string)
        .arg("full.ts")
        .output()
        .unwrap();
    println!("Merge done, fixing up the transport stream with ffmpeg.");
    Command::new("ffmpeg")
        .arg("-i")
        .arg("full.ts")
        .arg("-c")
        .arg("copy")
        .arg("full.final.ts")
        .output()
        .unwrap();
    println!("Fixing done, multiplexing resources into Matroska.");
    Command::new("mkvmerge.exe").arg("@options.json").arg("--output").arg(out).output().unwrap();
    println!("Finished your file is ready!")
}

async fn generate_subs(sub_url: String) {
    println!("Generating modified sub file.");
    let mut header_file = OpenOptions::new().read(true).open("subtitle_header_mod.ass").unwrap();
    let mut header: String = String::new();
    header_file.read_to_string(&mut header).unwrap();
    let sub_file = reqwest::get(&sub_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .split("\r\n")
        .skip(28)
        .collect::<Vec<&str>>()
        .join("\n");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(format!("en-us.ass"))
        .unwrap();
    file.write_all(&header.as_bytes()).unwrap();
    file.write_all(&sub_file.as_bytes()).unwrap();
    println!("Generated sub file with modified fonts.");
}

pub async fn download(hls_uri: &str, sub_uri: &str, quality: String, output_file_name: &str, download_thread: usize) {

    for x in parse_master(hls_uri).await.unwrap().variants {
        if x.resolution.unwrap_or(String::new()) == quality {
            let segments = &parse_playlist(&x.uri).await.unwrap();
            let client = reqwest::Client::new();
            let mut uris: Vec<String> = Vec::new();
            let segment = &segments.segments[0];
            uris.push(segment.uri.clone());
            let key_uri = &segment.key.as_ref().unwrap().uri.as_ref().unwrap();
            let r = reqwest::get(&key_uri.to_string()).await.unwrap().bytes().await.unwrap();
            let key: &[u8; 16] = pop(&r[..16]);
            let iv = b"0000000000000000";
            let cipher = Cipher::new_128(key);
            println!("Segments: {}", &segments.segments.len());
            for segment in &segments.segments[1..] {
                uris.push(segment.uri.clone());
            }
            let mut i = 0;
            // parallel download also not from stack overflow
            let bodies = stream::iter(uris)
                .map(|url| {
                    let client = client.clone();
                    i += 1;
                    tokio::spawn(async move {
                        let resp = client.get(url).send().await.unwrap();
                        SegDownloaded {
                            file: resp.bytes().await.unwrap(),
                            part_number: i.clone(),
                        }
                    })
                })
                .buffer_unordered(download_thread);
            let done = Arc::new(Mutex::new(1));
            let segment_count: usize = segments.segments.len();

            bodies
                .for_each(|b| async {
                    match b {
                        Ok(segment) => {
                            let mut lock = done.lock().await;
                            print!("\rDownloading - {}/{} segments", *lock, segment_count);
                            io::stdout().flush().unwrap();
                            // println!("Downloaded segment #{}", segment.part_number);
                            // println!("Segment #{} - {} bytes", segment.part_number, segment.file.len());
                            let decrypted: &[u8] = &cipher.cbc_decrypt(iv, &segment.file[..])[..];
                            let mut file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open(format!("seg.{}.ts", segment.part_number))
                                .unwrap();
                            file.write_all(decrypted).unwrap();
                            *lock += 1;

                        }
                        Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
                    }
                })
                .await;
            println!("Video download done!");
            generate_subs(sub_uri.to_string()).await;
            remux(output_file_name, segments.segments.len() as u32).await;
        }
    }
}
