use crate::models::SurrDB;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};

pub const VERSION: &str = "0.5-NotStable";

/// Print the banner
pub fn banner() {
    println!("######  {}  ######", "SurrealCLI".yellow());
    println!("Type `.help` for help menu.");
    println!("v {}", VERSION);
}

/// Print interactive help menu
pub fn help() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["COMMAND", "DESCRIPTION"]);

    table.add_row(vec![".help", "Show help menu"]);
    table.add_row(vec![".options", "Env variables"]);
    table.add_row(vec![".set", "Set variable"]);
    table.add_row(vec![".save", "Save profile|query"]);
    table.add_row(vec![".show", "Show profiles|queries"]);
    table.add_row(vec![".delete", "Delete profile|query"]);
    table.add_row(vec![".run", "Run profile|query"]);

    println!();
    println!("{}", table);
    println!();
}

/// Print command-line help menu
pub fn help_cmd() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["COMMAND", "DESCRIPTION", "DEFAULT"]);

    table.add_row(vec!["-u", "Username", "root"]);
    table.add_row(vec!["-p", "Password", "hide password"]);
    table.add_row(vec!["-host", "Database address \"IP:PORT\"", "0.0.0.0:80"]);
    table.add_row(vec!["-NS", "Namespace", "surr"]);
    table.add_row(vec!["-DB", "Database", "surr"]);
    table.add_row(vec!["-sc", "Schema", "http"]);
    table.add_row(vec!["-profile", "Connect to a profile", "none"]);
    table.add_row(vec!["-t", "Connection timeout", "5"]);
    table.add_row(vec!["-pretty", "Pretty output", "true"]);
    table.add_row(vec!["-comp", "Number of suggestions", "5"]);

    println!();
    println!("{}", table);
    println!();
}

/// Print current options
impl SurrDB {
    pub fn show_options(&self) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["VARIABLE", "VALUE"]);

        table.add_row(vec!["Host", &self.host]);
        table.add_row(vec!["User", &self.user]);
        table.add_row(vec!["Namespace", &self.namespace]);
        table.add_row(vec!["Database", &self.database]);
        table.add_row(vec!["Schema", &self.schema]);
        table.add_row(vec!["Pretty", &self.pretty.to_string()]);
        table.add_row(vec!["Timeout", &format!("{}s", self.timeout)]);
        table.add_row(vec!["Suggestion", &self.comple.to_string()]);

        println!();
        println!("{}", table);
        println!();
    }
}
