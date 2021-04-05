use futures::{stream, StreamExt};
use libaes::Cipher;
use m3u8_rs::playlist::{MasterPlaylist, MediaPlaylist, Playlist};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::{create_dir_all, remove_dir_all, remove_file, OpenOptions};
use std::io::prelude::*;
use std::io::{self};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

struct SegDownloaded {
    part_number: u128,
    file: bytes::Bytes,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Resume {
    finished_segments: Vec<u128>,
}

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

async fn cleanup(out: &str) {
    remove_dir_all(format!("Downloads/{}", out)).unwrap();
}

async fn remux(out: &str, segments: u32) {
    let mut segment_string = format!("seg.1.ts");
    for i in 2..segments {
        segment_string = format!("{}+seg.{}.ts", segment_string, i);
    }
    println!("Merging transport stream files.");
    merge(out, segments).await;
    // println!("Fixing up the transport stream with ffmpeg.");
    // Command::new("ffmpeg")
    //     .arg("-i")
    //     .arg(&format!("{}/full.ts", out))
    //     .arg("-c")
    //     .arg("copy")
    //     .arg(&format!("{}/full.final.ts", out))
    //     .output()
    //     .unwrap();
    println!("Multiplexing resources into Matroska.");
    Command::new("mkvmerge.exe")
        .args(vec![
            "--language",
            "0:und",
            "--language",
            "1:ja",
            "(",
            &format!("{}/full.ts", out),
            ")",
            "--language",
            "0:en",
            "--track-name",
            "0:ENG Subs",
            "(",
            &format!("{}/en-us.ass", out),
            ")",
            "--attachment-name",
            "Lato-Medium.ttf",
            "--attachment-mime-type",
            "application/x-truetype-font",
            "--attach-file",
            "fonts/Lato-Medium.ttf",
            "--attachment-name",
            "Lato-MediumItalic.ttf",
            "--attachment-mime-type",
            "application/x-truetype-font",
            "--attach-file",
            "fonts/Lato-MediumItalic.ttf",
            "--attachment-name",
            "times.ttf",
            "--attachment-mime-type",
            "application/x-truetype-font",
            "--attach-file",
            "fonts/times.ttf",
            "--attachment-name",
            "trebuc.ttf",
            "--attachment-mime-type",
            "application/x-truetype-font",
            "--attach-file",
            "fonts/trebuc.ttf",
            "--attachment-name",
            "arial.ttf",
            "--attachment-mime-type",
            "application/x-truetype-font",
            "--attach-file",
            "fonts/arial.ttf",
            "--attachment-name",
            "comic.ttf",
            "--attachment-mime-type",
            "application/x-truetype-font",
            "--attach-file",
            "fonts/comic.ttf",
            "--track-order",
            "0:0,0:1,1:0",
            "--output",
            &format!("{}.mkv", out),
        ])
        .output()
        .unwrap();
    println!("Finished your file is ready!")
}

#[cfg(target_os = "windows")]
async fn merge(out: &str, segments: u32) {
    let mut full = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(format!("{}/full.ts", out))
        .unwrap();
    for segment in 1..segments {
        let mut file = OpenOptions::new().read(true).open(format!("{}/seg.{}.ts", out, segment)).unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        full.write_all(&data[..]).unwrap();
        print!("\rMerging {}%", (segment / segments) * 100);
        io::stdout().flush().unwrap();
    }
    print!("\rMerging done.\n");
}

#[cfg(not(target_os = "windows"))]
async fn merge(out: &str, segments: u32) {
    let mut args = Vec::new();
    for i in 1..segments {
        args.push(format!("{}/seg.{}.ts", out, i));
    }
    let output = Command::new("cat").args(args).output().unwrap();
    let mut file = OpenOptions::new().write(true).create(true).open(format!("{}/full.ts", out)).unwrap();
    file.write_all(&output.stdout).unwrap();
    println!("Merging done.")
}

async fn generate_subs(sub_url: String, output_file_name: &str) {
    remove_file(format!("Downloads/{}/en-us.ass", output_file_name)).unwrap_or(());
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
        .open(format!("Downloads/{}/en-us.ass", output_file_name))
        .unwrap();
    file.write_all(&header.as_bytes()).unwrap();
    file.write_all(&sub_file.as_bytes()).unwrap();
    println!("Generated sub file with modified fonts.");
}

async fn save_state(resume: &Resume, out: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("Downloads/{}/state.json", out))
        .unwrap();
    file.write_all(&serde_json::to_string(&resume).unwrap().into_bytes()[..]).unwrap();
}

async fn load_state(out: &str) -> Resume {
    if !Path::new(&format!("Downloads/{}/state.json", out)).exists() {
        Resume { finished_segments: Vec::new() }
    } else {
        let mut file = OpenOptions::new().read(true).open(format!("Downloads/{}/state.json", out)).unwrap();
        let mut data: String = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str::<Resume>(&data).unwrap()
    }
}

pub async fn download(hls_uri: &str, sub_uri: &str, quality: String, output_file_name: &str, download_thread: usize) {
    for x in parse_master(hls_uri).await.unwrap().variants {
        if x.resolution.unwrap_or(String::new()) == quality {
            let resume = load_state(&output_file_name).await;
            create_dir_all(format!("Downloads/{}", output_file_name)).unwrap();
            let segments = &parse_playlist(&x.uri).await.unwrap();
            let client = reqwest::Client::new();
            let mut uris: Vec<String> = Vec::new();
            let segment = &segments.segments[0];
            uris.push(segment.uri.clone());
            let key_uri = &segment.key.as_ref().unwrap().uri.as_ref().unwrap();
            let r = reqwest::get(&key_uri.to_string()).await.unwrap().bytes().await.unwrap();
            let key: &[u8; 16] = pop(&r[..16]);
            // let iv = b"0000000000000000";
            let cipher = Cipher::new_128(key);
            println!("Segments: {}", &segments.segments.len());
            for segment in &segments.segments[1..] {
                uris.push(segment.uri.clone());
            }
            let mut i = 0;
            // parallel download also not from stack overflow
            let bodies = stream::iter(uris)
                .map(|url| {
                    i += 1;
                    if resume.finished_segments.contains(&i) {
                        tokio::spawn(async move {
                            SegDownloaded {
                                file: bytes::Bytes::new(),
                                part_number: 0,
                                uri: String::new(),
                            }
                        })
                    } else {
                        let client = client.clone();
                        tokio::spawn(async move {
                            let resp = client.get(&url).send().await.unwrap();
                            SegDownloaded {
                                file: resp.bytes().await.unwrap(),
                                part_number: i.clone(),
                                uri: url.clone().to_string(),
                            }
                        })
                    }
                })
                .buffer_unordered(download_thread);
            let done = Arc::new(Mutex::new(1));
            let segment_count: usize = segments.segments.len();
            let locked_resume = Arc::new(Mutex::new(resume.clone()));
            bodies
                .for_each(|b| async {
                    match b {
                        Ok(segment) => {
                            let mut lock = done.lock().await;
                            let mut res = locked_resume.lock().await;
                            print!("\rDownloading - {}/{} segments", *lock, segment_count);
                            io::stdout().flush().unwrap();
                            if segment.part_number != 0 {
                                let mut data = segment.file;
                                (*res).finished_segments.push(segment.part_number);
                                save_state(&*res, &output_file_name).await;
                                while &data[..].len() < &128 {
                                    println!(
                                        "\nSeems like segment #{} is corrupted, its only {} bytes long! Trying to redownload it.",
                                        &segment.part_number,
                                        &data[..].len()
                                    );
                                    data = client.get(&segment.uri).send().await.unwrap().bytes().await.unwrap();
                                }
                                let iv = &segment.part_number.to_be_bytes();
                                let decrypted: &[u8] = &cipher.cbc_decrypt(iv, &data[..])[..];
                                let mut file = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .open(format!("Downloads/{}/seg.{}.ts", output_file_name, segment.part_number))
                                    .unwrap();
                                file.write_all(decrypted).unwrap();
                            }
                            *lock += 1;
                        }
                        Err(e) => eprintln!("\nGot a tokio::JoinError: {}", e),
                    }
                })
                .await;
            println!("\nVideo download done!");
            generate_subs(sub_uri.to_string(), output_file_name).await;
            remux(&format!("Downloads/{}", output_file_name), segments.segments.len() as u32).await;
            cleanup(output_file_name).await; // cleanup the leftover files
        }
    }
}
