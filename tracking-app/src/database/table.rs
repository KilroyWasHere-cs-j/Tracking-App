#![allow(dead_code)]
use std::collections::HashMap;

/// Repersents a table in a database.
/// # Fields
/// * `name` - The name of the table.
/// * `id` - The id of the table.
/// * `table` - The table of the
/// # Example
/// ```
/// use no_sql_database::database::table::Table;
/// let mut table = Table::new(String::from("test"), 0);
/// ```
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    name: String,
    id: u64,
    table: HashMap<u64, String>,
}

impl Table {
    /// Creates a new table.
    /// # Arguments
    /// * `name` - The name of the table.
    /// * `id` - The id of the table.
    /// # Returns
    /// * `Table` - Returns a new table.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// ```
    pub fn new(name: String, id: u64) -> Table {
        Table {
            name,
            id,
            table: HashMap::new(),
        }
    }

    /// Inserts a new value into the table.
    /// # Arguments
    /// * `id` - The id of the value.
    /// * `value` - The value to be inserted.
    /// # Returns
    /// * `()` - Returns nothing.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// table.insert(0, String::from("test"));
    /// ```
    pub fn insert(&mut self, id: u64, value: String) {
        self.table.insert(id, value);
    }

    /// Gets a value from the table.
    /// # Arguments
    /// * `id` - The id of the value.
    /// # Returns
    /// * `Option<&String>` - Returns an option of a reference to the value.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// table.insert(0, String::from("test"));
    /// let value = table.get(0);
    /// ```
    pub fn get(&self, id: u64) -> Option<&String> {
        self.table.get(&id)
    }

    /// Removes a value from the table.
    /// # Arguments
    /// * `id` - The id of the value.
    /// # Returns
    /// * `Option<String>` - Returns an option of the value.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// table.insert(0, String::from("test"));
    /// let value = table.remove(0);
    /// ```
    pub fn remove(&mut self, id: u64) -> Option<String> {
        self.table.remove(&id)
    }

    /// Removes all values from the table.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `()` - Returns nothing.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// table.insert(0, String::from("test"));
    /// table.remove_all();
    /// ```
    pub fn remove_all(&mut self) {
        self.table.clear();
    }

    pub fn query_by_key(&self, id: u64) -> Option<&String> {
        self.table.get(&id)
    }

    // TODO - Add regex search
    pub fn query_by_value(&self, value: String) -> Option<&String> {
        for (_key, val) in &self.table {
            if val.contains(&value) {
                return Some(val);
            }
        }
        None
    }

    /// Updates a value in the table.
    /// # Arguments
    /// * `id` - The id of the value.
    /// * `value` - The new value.
    /// # Returns
    /// * `Option<String>` - Returns an option of the old value.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// table.insert(0, String::from("test"));
    /// let value = table.update(0, String::from("test2"));
    /// ```
    pub fn update(&mut self, id: u64, value: String) -> Option<String> {
        self.table.insert(id, value)
    }

    /// Gets the name of the table.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `&String` - Returns a reference to the name of the table.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// let name = table.get_name();
    /// ```
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the id of the table.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `u64` - Returns the id of the table.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// let id = table.get_id();
    /// ```
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// Prints the table.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `()` - Returns nothing.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// table.print_table();
    /// ```

    // ! Design it so that it prints the table in a nice format
    // ! In numarical order
    pub fn print_table(&self) {
        println!("Name: {}", self.name);
        println!("Id: {}", self.id);
        for (key, value) in &self.table {
            println!("{}: {}", key, value);
        }
    }
    /// Gets the table.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `&HashMap<u64, String>` - Returns a reference to the table.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// let table = table.get_table();
    /// ```
    pub fn get_table(&self) -> &HashMap<u64, String> {
        &self.table
    }

    /// Gets the table.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `&mut HashMap<u64, String>` - Returns a mutable reference to the table.
    /// # Example
    /// ```
    /// use no_sql_database::database::table::Table;
    /// let mut table = Table::new(String::from("test"), 0);
    /// let table = table.get_table_mut();
    /// ```
    pub fn get_table_mut(&mut self) -> &mut HashMap<u64, String> {
        &mut self.table
    }
}

// Kilroy Was Here
