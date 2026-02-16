use crate::models::SurrDB;
use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

/// Global database connection
static mut DB_CONN: Option<Connection> = None;

/// Get the database file path
fn get_db_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
    Ok(home.join(".local").join("surrcli.db"))
}

/// Initialize database file and create tables
pub fn db_file_init() -> Result<()> {
    let db_path = get_db_path()?;

    // Create .local directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create database file if it doesn't exist
    if !db_path.exists() {
        crate::utils::print_err("Error locating local database. Creating one!");
        std::fs::File::create(&db_path)?;
        crate::utils::print_suc(&format!("Database created: {}", db_path.display()));
    }

    let conn = Connection::open(&db_path)?;
    db_table_set(&conn)?;

    unsafe {
        DB_CONN = Some(conn);
    }

    Ok(())
}

/// Create all database tables
fn db_table_set(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Profile (
            pid INTEGER PRIMARY KEY AUTOINCREMENT,
            Idx TEXT NOT NULL,
            Host TEXT NOT NULL,
            Sch TEXT NOT NULL,
            DBUser TEXT NOT NULL,
            NS TEXT NOT NULL,
            DB TEXT NOT NULL,
            Date TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Sess (
            sid INTEGER PRIMARY KEY AUTOINCREMENT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS SQuery (
            qid INTEGER PRIMARY KEY AUTOINCREMENT,
            Idx TEXT,
            Query TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS SOut (
            qid INTEGER PRIMARY KEY AUTOINCREMENT,
            Idx TEXT,
            Query TEXT
        )",
        [],
    )?;

    Ok(())
}

/// Get database connection
fn get_conn() -> Result<&'static Connection> {
    unsafe {
        // Use &raw const to avoid the static_mut_refs warning
        let ptr = &raw const DB_CONN;
        if ptr.is_null() {
            return Err(anyhow::anyhow!("Database not initialized"));
        }
        // Dereference as Option<&Connection>
        match &*ptr {
            Some(conn) => Ok(conn),
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }
}

/// Check if profile index exists
pub fn db_valid_index(idx: &str) -> Result<bool> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT pid FROM Profile WHERE Idx = ?")?;

    let mut rows = stmt.query([idx])?;
    let exists = rows.next()?.is_some();

    Ok(exists)
}

/// Save current configuration as a profile
impl SurrDB {
    pub fn db_save_profile(&self, name: &str) -> Result<()> {
        if db_valid_index(name)? {
            crate::utils::print_err("Profile name exists.");
            return Ok(());
        }

        let conn = get_conn()?;
        conn.execute(
            "INSERT INTO Profile (Idx, Host, Sch, DBUser, NS, DB, Date) VALUES (?, ?, ?, ?, ?, ?, datetime('now', 'localtime'))",
            [name, &self.host, &self.schema, &self.user, &self.namespace, &self.database],
        )?;

        crate::utils::print_suc("Profile saved.");
        Ok(())
    }
}

/// Show all saved profiles
pub fn db_show_profiles() -> Result<()> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare(
        "SELECT pid, Idx, Host, Sch, DBUser, NS, DB, Date FROM Profile"
    )?;

    let mut rows = stmt.query([])?;

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.set_header(vec!["ID", "NAME", "HOST", "PROTOCOL", "USER", "NAMESPACE", "DATABASE", "CREATION DATE"]);

    while let Some(row) = rows.next()? {
        let pid: i32 = row.get(0)?;
        let idx: String = row.get(1)?;
        let host: String = row.get(2)?;
        let sch: String = row.get(3)?;
        let dbuser: String = row.get(4)?;
        let ns: String = row.get(5)?;
        let db: String = row.get(6)?;
        let date: String = row.get(7)?;

        table.add_row(vec![
            pid.to_string(),
            idx,
            host,
            sch,
            dbuser,
            ns,
            db,
            date,
        ]);
    }

    println!();
    println!("{}", table);
    println!();

    Ok(())
}

/// Delete a profile by index
pub fn db_drop_idx(idx: &str) -> Result<()> {
    if !db_valid_index(idx)? {
        crate::utils::print_err("Profile does not exist.");
        return Ok(());
    }

    let conn = get_conn()?;
    conn.execute("DELETE FROM Profile WHERE Idx = ?", [idx])?;
    crate::utils::print_suc(&format!("{} deleted.", idx));
    Ok(())
}

/// Load profile by index
impl SurrDB {
    pub fn db_set_profile_by_idx(&mut self, idx: &str) -> Result<()> {
        if !db_valid_index(idx)? {
            crate::utils::print_err("No profile.");
            return Ok(());
        }

        let conn = get_conn()?;
        let mut stmt = conn.prepare("SELECT Host, Sch, DBUser, NS, DB FROM Profile WHERE Idx = ?")?;

        let mut rows = stmt.query([idx])?;
        if let Some(row) = rows.next()? {
            self.host = row.get::<usize, String>(0)?;
            self.schema = row.get::<usize, String>(1)?;
            self.user = row.get::<usize, String>(2)?;
            self.namespace = row.get::<usize, String>(3)?;
            self.database = row.get::<usize, String>(4)?;
        }

        Ok(())
    }
}

/// Check if query index exists
pub fn db_valid_query_index(idx: &str) -> Result<bool> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT qid FROM SQuery WHERE Idx = ?")?;

    let mut rows = stmt.query([idx])?;
    let exists = rows.next()?.is_some();

    Ok(exists)
}

/// Save current query
impl SurrDB {
    pub fn db_save_query(&self, name: &str) -> Result<()> {
        if self.query.is_empty() {
            crate::utils::print_err("No query to save.");
            return Ok(());
        }

        if db_valid_query_index(name)? {
            crate::utils::print_err("Query name exists.");
            return Ok(());
        }

        let conn = get_conn()?;
        conn.execute(
            "INSERT INTO SQuery (Idx, Query) VALUES (?, ?)",
            [name, &self.query],
        )?;

        crate::utils::print_suc("Query saved.");
        Ok(())
    }
}

/// Show all saved queries
pub fn db_show_queries() -> Result<()> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT qid, Idx, Query FROM SQuery")?;

    let mut rows = stmt.query([])?;

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.set_header(vec!["ID", "NAME", "QUERY"]);

    while let Some(row) = rows.next()? {
        let qid: i32 = row.get(0)?;
        let idx: String = row.get(1)?;
        let query: String = row.get(2)?;

        table.add_row(vec![
            qid.to_string(),
            idx,
            query,
        ]);
    }

    println!();
    println!("{}", table);
    println!();

    Ok(())
}

/// Delete a query by index
pub fn db_drop_query_idx(idx: &str) -> Result<()> {
    if !db_valid_query_index(idx)? {
        crate::utils::print_err("Query does not exist.");
        return Ok(());
    }

    let conn = get_conn()?;
    conn.execute("DELETE FROM SQuery WHERE Idx = ?", [idx])?;
    crate::utils::print_suc(&format!("{} deleted.", idx));
    Ok(())
}

/// Get query by index
pub fn db_get_query_by_idx(idx: &str) -> Result<String> {
    if !db_valid_query_index(idx)? {
        crate::utils::print_err("Query does not exist.");
        return Err(anyhow::anyhow!("Query not found"));
    }

    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT Query FROM SQuery WHERE Idx = ?")?;

    let mut rows = stmt.query([idx])?;
    if let Some(row) = rows.next()? {
        let query: String = row.get(0)?;
        Ok(query)
    } else {
        Err(anyhow::anyhow!("Query not found"))
    }
}
