use db_core::{Column, Table};
use std::{collections::HashMap, error::Error};

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
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
