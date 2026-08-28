use db_core::{Column, Row};

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
