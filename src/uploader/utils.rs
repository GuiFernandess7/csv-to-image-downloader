use reqwest::ResponseBuilderExt;
use serde_json::{Value};
use std::fs::{File, create_dir_all};
use std::io::Read;

use reqwest::blocking::get;
use std::io::copy;
use std::io::{self, Write};
use image::io::Reader as ImageReader;
use std::error::Error;
use std::path::Path;

pub fn get_images_url(file_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::open(file_path)?;
    let mut data: String = String::new();
    file.read_to_string(&mut data)?;

    let json_data: Value = serde_json::from_str(&data)?;

    if let Value::Array(items) = json_data {
        for item in items {
            if let Some(url) = item.get("image_url"){
                let url_str = url.as_str().unwrap_or_default();
                if let Err(e) = download_image(url_str, output_path){
                    eprint!("Erro ao baixar imagem de URL: '{}': '{}' \n", url_str, e)
                }
            }
        }
    }
    Ok(())
}

fn download_image(url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(output_path);
    if !path.exists() {
        create_dir_all(path)?;
    }

    let file_name = url.split('/').last().unwrap_or("image.jpg");
    let output_file_path = Path::new(output_path).join(format!("{}.jpg", file_name));
    let response = get(url)?;

    if response.status().is_success() {
        let mut dest = File::create(&output_file_path)?;
        let bytes = response.bytes()?;

        io::copy(&mut bytes.as_ref(), &mut dest)?;

        match ImageReader::open(output_file_path)?.decode() {
            Ok(img) => {
                println!("Imagem carregada com sucesso");
            },
            Err(e) => {
                eprintln!("Erro ao carregar imagem: {}", e);
            }
        }

        Ok(())
    } else {
        Err(format!("Falha ao baixar a imagem: {}", response.status()).into())
    }
}