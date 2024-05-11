
use std::{borrow::BorrowMut, collections::{BTreeMap, HashMap}, ops::{Deref, DerefMut}, usize};

use serde::{Deserialize, Serialize};



#[derive(Debug, Clone)]
pub enum CellValue {
  Int(isize),
  UInt(usize),
  Float(f64),
}

impl PartialEq for CellValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (CellValue::Int(a), CellValue::Int(b)) => a == b,
      (CellValue::UInt(a), CellValue::UInt(b)) => a == b,
      (CellValue::Float(a), CellValue::Float(b)) => a == b,
      _ => false,
    }
  }
}

// impl Copy for CellValue {
  
// }

pub struct SpreadSheetCells {
  data: BTreeMap<String, CellValue>,
}

// impl Deref for SpreadSheetCells {
//   type Target = HashMap<String, CellValue>;

//   fn deref(&self) -> &Self::Target {
//       &self.data
//   }
// }

// impl DerefMut for SpreadSheetCells {
//   fn deref_mut(&mut self) -> &mut Self::Target {
//       &mut self.data
//   }
// }

impl SpreadSheetCells {
  pub fn new() -> Self {
    SpreadSheetCells {
        data: BTreeMap::new(),
    }
  }

  pub fn set(&mut self, key: &str, value: CellValue) {
    self.data.insert(key.to_string().to_ascii_uppercase(), value);
  }

  pub fn get(&self, key: &str) -> Option<&CellValue> {
    self.data.get(&key.to_ascii_uppercase())
  }

  pub fn get_all(&self) -> Vec<(String, CellValue)> {
    self.data.clone().into_iter().collect::<Vec<_>>()
  }

  pub fn get_range(&self, range: &str) -> Vec<(String, CellValue)> {
    let mut result: Vec<(String, CellValue)> = Vec::new();
    let start_end: Vec<&str> = range.split(':').collect(); // Разбиваем диапазон на начальную и конечную ячейки
    if start_end.len() == 2 {
      if let (Some(start), Some(end)) = (start_end[0].chars().next(), start_end[1].chars().next()) {

        let start_letter = start.to_ascii_uppercase(); // Переводим первый символ начальной ячейки в верхний регистр
        let end_letter = end.to_ascii_uppercase(); // Переводим первый символ конечной ячейки в верхний регистр
        
        if let (Some(start_num), Some(end_num)) = (start_end[0][1..].parse::<usize>().ok(), start_end[1][1..].parse::<usize>().ok()) {
          // Получаем числовое значение строк начала и конца диапазона

          for (key, value) in &self.data {
            if let (Some(curr_letter), Some(curr_num)) = (key.chars().next(), key[1..].parse::<usize>().ok()) {
              if curr_letter <= end_letter && curr_num <= end_num && curr_letter >= start_letter && curr_num >= start_num {
                // Проверяем, находится ли текущая ячейка внутри диапазона

                result.push((String::from(key), value.clone()));
              }
            }
          }
        }
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut db = SpreadSheetCells::new();
        db.set("A1", CellValue::Int(555));
        assert_eq!(db.get("a1"), Some(&CellValue::Int(555)));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db = SpreadSheetCells::new();
        assert_eq!(db.get("B2"), None);
    }

    #[test]
    fn test_get_range() {
        let mut db = SpreadSheetCells::new();
        db.set("A1", CellValue::Int(555));
        db.set("B1", CellValue::Float(555.555));
        db.set("B2", CellValue::UInt(321));
        db.set("C3", CellValue::Int(999));
        db.set("D4", CellValue::Int(111));
        db.set("C5", CellValue::Int(333));

        let mut result = db.get_range("B1:D4");

        let D4 = String::from("D4");
        let B1 = String::from("B1");
        let B2 = String::from("B2");
        let C3 = String::from("C3");

        let mut expected: Vec<_> = vec![
          (D4, CellValue::Int(111)),
          (B1, CellValue::Float(555.555)), 
          (B2, CellValue::UInt(321)), 
          (C3, CellValue::Int(999)), 
        ];

        expected.sort_by_key(|&(ref s, _)| s.clone());
        result.sort_by_key(|&(ref s, _)| s.clone());
        
        assert_eq!(result, expected);
    }
}