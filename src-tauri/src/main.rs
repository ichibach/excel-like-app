// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]






use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, sync::Mutex};

use calamine::{open_workbook, Data, Reader, Xlsx};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

mod xlsx_core;
use tauri::State;
use xlsx_core::{SpreadSheetCells, CellValue};


struct Storage {
    store: Mutex<SpreadSheetCells>,
}

struct AppState (Rc<RefCell<SpreadSheetCells>>);

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

impl<'de> Deserialize<'de> for CellValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: Deserializer<'de>,
  {
      struct CellValueVisitor;

      impl<'de> de::Visitor<'de> for CellValueVisitor {
          type Value = CellValue;

          fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
              formatter.write_str("an integer, unsigned integer, or float")
          }

          fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
          where
              E: de::Error,
          {
              Ok(CellValue::Int(value as isize))
          }

          fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
          where
              E: de::Error,
          {
              Ok(CellValue::UInt(value as usize))
          }

          fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
          where
              E: de::Error,
          {
              Ok(CellValue::Float(value))
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
              E: de::Error,
          {
              // При желании, можно также обработать парсинг строки в числовой тип
              Err(de::Error::custom("expected an integer, unsigned integer, or float"))
          }
      }

      deserializer.deserialize_any(CellValueVisitor)
  }
}


impl Serialize for CellValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            CellValue::Float(f) => serializer.serialize_f64(*f),
            CellValue::Int(i) => serializer.serialize_i64((*i).try_into().unwrap()),
            CellValue::UInt(ui) => serializer.serialize_u64((*ui).try_into().unwrap()),
            _ => unimplemented!("Serialization for other data types not implemented"),
        }
    }
}


#[tauri::command]
fn set_cell_value(key: &str, value: CellValue, storage: State<Storage>) {
    storage.store.lock().unwrap().set(key, value);
}

#[tauri::command]
fn get_range(range: &str, storage: State<Storage>) -> Vec<(String, CellValue)> {
    // state.get_range(range)
    storage.store.lock().unwrap().get_range(range)
}

fn main() {
    tauri::Builder::default()
        .manage(Storage { store: Mutex::new(SpreadSheetCells::new())})
        .invoke_handler(tauri::generate_handler![set_cell_value, get_range])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
