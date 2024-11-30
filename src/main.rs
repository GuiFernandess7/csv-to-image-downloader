#![allow(unused)]
mod converter;
mod uploader;
mod args;

use std::fs::File;
use std::io::{self, Write};
use std::error::Error;
use std::path::Path;
use std::str;
use args::{RCArgs, Modes};

use clap::Parser;
use converter::model::Record;
use converter::utils::{read_csv_file, push_records, write_json};
use uploader::utils::{get_images_url};

fn main() -> Result<(), Box<dyn Error>> {
    let args = RCArgs::parse();
    let mut records: Vec<Record> = Vec::new();

    match args.mode {
        Modes::Convert { file_path, output_folder } => {
            let mut file_reader = read_csv_file(&file_path.as_str())?;
            push_records(&mut records, &mut file_reader);
            write_json(&records, &output_folder)?;
        }
        Modes::Download { file_data_path, output_folder } => {
            println!("Modo de download ativado para o arquivo: {}", file_data_path);
            get_images_url(file_data_path.as_str(), output_folder.as_str())?;
        }
    }

    Ok(())
}