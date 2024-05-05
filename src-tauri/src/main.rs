// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use calamine::{open_workbook, Data, Reader, Xlsx};
use serde::{Serialize, Serializer};


// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

struct WrappedData(Data);

impl Serialize for WrappedData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Data::String(s) => serializer.serialize_str(&s),
            Data::Float(f) => serializer.serialize_f64(*f),
            Data::Int(i) => serializer.serialize_i64(*i),
            Data::Bool(b) => serializer.serialize_bool(*b),
            Data::Empty => serializer.serialize_unit(),
            _ => unimplemented!("Serialization for other data types not implemented"),
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_file(file: &str) -> Result<Vec<(usize, usize, WrappedData)>,Error> {

    let mut workbook: Xlsx<_> = open_workbook(file).expect("failed to try read file");

    let mut data:Vec<(usize, usize, WrappedData)> = Vec::new();

    if let Some(Ok(r)) = workbook.worksheet_range_at(0) {
        for cell in r.cells() {
            data.push((cell.0, cell.1, WrappedData(cell.2.clone())));
            println!("row={:?}, coll={:?}, value={:?}", cell.0, cell.1, cell.2);
        }
    }

    Ok(data)   
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
