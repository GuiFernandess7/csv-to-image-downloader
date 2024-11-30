use csv::{Reader, ReaderBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::error::Error;
use std::path::Path;
use std::str;

use super::model::Record;

pub fn read_csv_file(path: &str) -> Result<Reader<File>, Box<dyn Error>>{
    let file_path = Path::new(path);
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    Ok((rdr))
}

pub fn push_records(records: &mut Vec<Record>, rdr: &mut Reader<File>) -> Result<(), Box<dyn Error>>{
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(())
}

pub fn write_json(records: &Vec<Record>, output_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_folder)?;
    serde_json::to_writer_pretty(file, &records)?;
    Ok(())
}