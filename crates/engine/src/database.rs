use db_core::{Column, Table};
use std::{collections::HashMap, error::Error};

use crate::command::Command;

#[derive(Debug)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(
        &mut self,
        table_name: &str,
        columns: Vec<Column>,
    ) -> Result<(), Box<dyn Error>> {
        if self.tables.contains_key(table_name) {
            return Err(format!("Table '{}' already exists", table_name).into());
        }

        let new_table = Table::new(table_name.to_string(), columns);
        self.tables.insert(table_name.to_string(), new_table);

        Ok(())
    }

    pub fn get_mut_table(&mut self, table_name: &str) -> Option<&mut Table> {
        self.tables.get_mut(table_name)
    }

    pub fn get_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.get(table_name)
    }

    pub fn execute(&mut self, command: Command) -> Result<(), Box<dyn Error>> {
        match command {
            Command::CreateTable {
                table_name,
                columns,
            } => self.create_table(&table_name, columns)?,
            Command::Insert {
                table_name,
                id,
                row,
            } => {
                let table = self
                    .get_mut_table(&table_name)
                    .ok_or(format!("Table '{}' not found", table_name))?;

                table.insert(id, row)?;
                println!("Row inserted into '{}' (ID: {}).", table_name, id);
            }
            Command::Select { table_name } => {
                let table = self
                    .get_table(&table_name)
                    .ok_or(format!("Table '{}' not found", table_name))?;

                println!("Data in table '{}'", table_name);

                for (id, row) in table.get_rows() {
                    println!("ID: {} | Values: {:?}", id, row);
                }
            }
        }
        Ok(())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
