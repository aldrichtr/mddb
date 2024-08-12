use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            name,
            fields: HashMap::new(),
            columns: HashMap::new(),
            select_columns: Vec::new(),
        }
    }

    pub fn insert(&mut self, column: String, value: DataType) {
        self.columns.entry(column).or_insert(Vec::new()).push(value);
    }

    pub fn select(&self, columns: Vec<&str>) -> String {
        let mut result = String::new();
        for col in columns {
            if let Some(values) = self.columns.get(col) {
                for value in values {
                    result.push_str(&format!("{:?} ", value));
                }
            }
            result.push('\n');
        }
        result
    }

}
