use std::fs::{create_dir_all, File};
use std::io::{copy};
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;

/// Ensure a SNAP text dataset exists under `data/<name>.txt`.
/// If missing, downloads `url_gz` and decompresses it.
pub fn ensure_snap_txt(name: &str, url_gz: &str) -> PathBuf {
    let data_dir = Path::new("data");
    let _ = create_dir_all(&data_dir);

    let txt_path = data_dir.join(format!("{}.txt", name));
    if txt_path.exists() {
        return txt_path;
    }

    let gz_path = data_dir.join(format!("{}.txt.gz", name));

    let client = Client::builder()
        .build()
        .expect("build HTTP client");
    let mut response = client
        .get(url_gz)
        .send()
        .and_then(|r| r.error_for_status())
        .expect("download dataset");

    let mut gz_file = File::create(&gz_path).expect("create .gz file");
    copy(&mut response, &mut gz_file).expect("write .gz file");

    let gz_file = File::open(&gz_path).expect("open .gz file");
    let mut decoder = GzDecoder::new(gz_file);
    let mut txt_file = File::create(&txt_path).expect("create .txt file");
    copy(&mut decoder, &mut txt_file).expect("decompress .gz to .txt");

    // Optional: keep the .gz for caching; comment next line to keep
    let _ = std::fs::remove_file(&gz_path);

    txt_path
}

/// Ensure a generic `.gz` is downloaded and decompressed to a target extension.
/// Returns the decompressed file path under `data/<name>.<out_ext>`.
pub fn ensure_gz_decompressed(name: &str, url_gz: &str, out_ext: &str) -> PathBuf {
    let data_dir = Path::new("data");
    let _ = create_dir_all(&data_dir);

    let out_path = data_dir.join(format!("{}.{}", name, out_ext));
    if out_path.exists() {
        return out_path;
    }

    let gz_path = data_dir.join(format!("{}.{}.gz", name, out_ext));

    let client = Client::builder()
        .build()
        .expect("build HTTP client");
    let mut response = client
        .get(url_gz)
        .send()
        .and_then(|r| r.error_for_status())
        .expect("download dataset");

    let mut gz_file = File::create(&gz_path).expect("create .gz file");
    copy(&mut response, &mut gz_file).expect("write .gz file");

    let gz_file = File::open(&gz_path).expect("open .gz file");
    let mut decoder = GzDecoder::new(gz_file);
    let mut out_file = File::create(&out_path).expect("create output file");
    copy(&mut decoder, &mut out_file).expect("decompress .gz");

    let _ = std::fs::remove_file(&gz_path);

    out_path
}


