use super::table::Table;
use std::{env, fs};
use std::fs::File;
use std::io::Write;

/// Struct that represents a database
/// # Arguments
/// * `name` - A String that holds the name of the database.
/// * `id` - A u64 that holds the id of the database.
/// * `database` - A Vec<Table> that holds the tables of the database.
/// * `ids` - A Vec<u64> that holds the ids of the tables of the database.
/// # Example
/// ```
/// use no_sql_database::database::db::Database;
/// let mut db = Database::new(String::from("test"), 0);
/// ```
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    name: String,
    id: u64,
    database: Vec<Table>,
    ids: Vec<u64>,
}

pub struct TruncDatabase(Vec<Database>);

impl Database {
    /// Creates a new database.
    /// # Arguments
    /// * `name` - A String that holds the name of the database.
    /// * `id` - A u64 that holds the id of the database.
    /// # Returns
    /// * `Database` - Returns a new database.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// ```
    pub fn new(name: String, id: u64) -> Database {
        Database {
            name,
            id,
            database: Vec::new(),
            ids: Vec::new(),
        }
    }

    /// Saves database to file
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `()` - Returns nothing.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// db.save();
    /// ```
    pub fn save(&mut self) {
        let file = File::create(format!("{} {}.db", Database::get_current_working_dir() + "/db_cache/", self.name)).unwrap();
        let file = serde_json::to_writer_pretty(file, self).unwrap();
    }

    /// Gets the current working directory
    /// # Arguments
    /// * None
    /// # Returns
    /// * String
    /// # Example
    /// ```
    /// let cwd = get_current_working_dir();
    /// ```
    #[allow(dead_code)]
    fn get_current_working_dir() -> String {
        env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    }

    /// Loads database from file
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `()` - Returns nothing.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// db.load();
    /// ```
    pub fn load(&mut self, file_name: String) -> std::io::Result<()> {
        match File::open(file_name) {
            Ok(file) => {
                let file = serde_json::from_reader(file)?;
                *self = file;
                Ok(())
            },
            Err(_) => {
                fs::create_dir(Database::get_current_working_dir() + "/db_cache")?;
                File::create(format!("{} {}.db", Database::get_current_working_dir() + "/db_cache/", self.name))?;
                let file = File::create(format!("{} {}.db", Database::get_current_working_dir() + "/db_cache/", self.name))?;
                let file = serde_json::from_reader(file)?;
                *self = file;
                Ok(())
            }
        }
    }

    /// Gets the table with the given id.
    /// # Arguments
    /// * `id` - A u64 that holds the id of the table.
    /// # Returns
    /// * `Option<&Table>` - Returns a reference to the table if it exists.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// let table = db.get_table(0);
    /// ```
    pub fn insert_table(&mut self, table: Table) {
        self.ids.push(table.get_id());
        self.database.push(table);
    }

    /// Gets the table with the given id.
    /// # Arguments
    /// * `id` - A u64 that holds the id of the table.
    /// # Returns
    /// * `Option<&Table>` - Returns a reference to the table if it exists.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// let table = db.get_table(0);
    /// ```
    pub fn get_table(&self, id: u64) -> Option<&Table> {
        self.database.iter().find(|&table| table.get_id() == id)
    }

    /// Gets the table with the given id.
    /// # Arguments
    /// * `id` - A u64 that holds the id of the table.
    /// # Returns
    /// * `Option<&mut Table>` - Returns a mutable reference to the table if it exists.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// let table = db.get_table_mut(0);
    /// ```
    pub fn get_table_mut(&mut self, id: u64) -> Option<&mut Table> {
        self.database.iter_mut().find(|table| table.get_id() == id)
    }

    /// Gets the name of the database.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `&String` - Returns a reference to the name of the database.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// let name = db.get_name();
    /// ```
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the id of the database.
    /// # Arguments
    /// * `()` - Takes no arguments.
    /// # Returns
    /// * `u64` - Returns the id of the database.
    /// # Example
    /// ```
    /// use no_sql_database::database::db::Database;
    /// let mut db = Database::new(String::from("test"), 0);
    /// let id = db.get_id();
    /// ```
    pub fn get_id(&self) -> u64 {
        self.id
    }
}
