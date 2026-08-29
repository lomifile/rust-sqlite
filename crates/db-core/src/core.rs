use std::{collections::BTreeMap, error::Error};

#[derive(Debug)]
pub enum Value {
    Text(String),
    Number(i64),
    Decimal(f64),
    Date(String),
}

#[derive(Debug)]
pub enum DataType {
    Text,
    Number,
    Decimal,
    Date,
}

#[derive(Debug)]
pub struct Column {
    name: String,
    data_type: DataType,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Self {
        Column { name, data_type }
    }
}

#[derive(Debug)]
pub struct Row {
    values: Vec<Value>,
}

impl Row {
    pub fn new(values: Option<Vec<Value>>) -> Self {
        match values {
            Some(values) => Row { values },
            None => Row { values: Vec::new() },
        }
    }
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: BTreeMap<i64, Row>,
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Table {
            columns,
            name,
            rows: BTreeMap::new(),
        }
    }

    fn validate_row_types(&self, row: &Row) -> Result<(), Box<dyn Error>> {
        for (col, val) in self.columns.iter().zip(row.values.iter()) {
            match (&col.data_type, val) {
                (DataType::Text, Value::Text(_)) => {}
                (DataType::Number, Value::Number(_)) => {}
                (DataType::Decimal, Value::Decimal(_)) => {}
                (DataType::Date, Value::Date(_)) => {}

                _ => return Err(format!("Type mismatch for column '{}'", col.name).into()),
            }
        }
        Ok(())
    }

    pub fn insert(&mut self, id: i64, row: Row) -> Result<(), Box<dyn Error>> {
        if row.values.len() != self.columns.len() {
            return Err("insert error: mismatched column count".into());
        }

        if self.rows.contains_key(&id) {
            return Err("insert error: duplicate id".into());
        }

        self.validate_row_types(&row)?;

        self.rows.insert(id, row);

        Ok(())
    }

    pub fn get_rows(&self) -> &BTreeMap<i64, Row> {
        &self.rows
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new(String::new(), Vec::new())
    }
}
