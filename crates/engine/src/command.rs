use std::error::Error;

use db_core::{Column, DataType, Row, Value};

#[derive(Debug)]
pub enum Command {
    CreateTable {
        table_name: String,
        columns: Vec<Column>,
    },
    Insert {
        table_name: String,
        id: i64,
        row: Row,
    },
    Select {
        table_name: String,
    },
}

pub fn parse_statement(input: &str) -> Result<Command, Box<dyn Error>> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.is_empty() {
        return Err("parse statemene error: empty statement".into());
    }

    match tokens[0].to_uppercase().as_str() {
        "SELECT" => {
            if tokens.len() < 4 || tokens[1] != "*" || tokens[2].to_uppercase() != "FROM" {
                return Err("parse statement error: invalid SELECT format. Expected: SELECT * FROM <table_name>".into());
            }

            Ok(Command::Select {
                table_name: tokens[3].to_string(),
            })
        }
        "INSERT" => {
            if tokens.len() < 4 || tokens[1].to_uppercase() != "INTO" {
                return Err("parse statement error: invalid INSERT format. Expected INSERT INTO <table_name> <id> <values>".into());
            }

            let table_name = tokens[2];
            let id = tokens[3].parse::<i64>()?;

            let mut values = Vec::new();

            for &val_str in &tokens[4..] {
                if let Ok(num) = val_str.parse::<i64>() {
                    values.push(Value::Number(num));
                } else if let Ok(dec) = val_str.parse::<f64>() {
                    values.push(Value::Decimal(dec));
                } else {
                    values.push(Value::Text(val_str.to_string()));
                }
            }

            Ok(Command::Insert {
                table_name: table_name.to_string(),
                id,
                row: Row::new(Some(values)),
            })
        }
        "CREATE" => {
            if tokens.len() < 3 || tokens[1].to_uppercase() != "TABLE" {
                return Err(
                    "parse statement error: Expected CREATE TABLE <name> <col1> <type1>...".into(),
                );
            }

            let table_name = tokens[2].to_string();

            let columns_tokens = &tokens[3..];

            if !columns_tokens.len().is_multiple_of(2) {
                return Err(
                    "parse statement error: columns must be defined as <name> <type> pairs".into(),
                );
            }

            let mut columns = Vec::new();

            for chunk in columns_tokens.chunks(2) {
                let col_name = chunk[0].to_string();
                let col_type_str = chunk[1].to_uppercase();

                let data_type = match col_type_str.as_str() {
                    "TEXT" => DataType::Text,
                    "NUMBER" => DataType::Number,
                    "DECIMAL" => DataType::Decimal,
                    "DATE" => DataType::Date,
                    _ => return Err(format!("Unknown data type: {}", col_type_str).into()),
                };

                columns.push(Column::new(col_name, data_type));
            }

            Ok(Command::CreateTable {
                table_name,
                columns,
            })
        }
        _ => Err(format!("Unknown command: {}", tokens[0]).into()),
    }
}
