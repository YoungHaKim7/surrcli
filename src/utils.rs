use crate::models::SurrDB;
use colored::Colorize;
use std::io;

/// Print error message with red [!] prefix
pub fn print_err(s: &str) {
    println!("{}- {}", "[!]".red().bold(), s);
}

/// Print success message with green [OK] prefix
pub fn print_suc(s: &str) {
    println!("{}- {}", "[OK]".green().bold(), s);
}

/// Pretty print JSON response
pub fn pretty_print(json_str: &str) -> Result<(), anyhow::Error> {
    let parsed: serde_json::Value = serde_json::from_str(json_str)?;
    let pretty = serde_json::to_string_pretty(&parsed)?;
    println!("{}", pretty);
    Ok(())
}

/// Print raw response
pub fn print_raw(s: &str) {
    println!("{}", s);
}

/// Set configuration variables
impl SurrDB {
    pub fn set_var(&mut self, var: &str, value: &str) {
        match var {
            "user" | "User" => {
                self.user = value.to_string();
                print_suc(&format!("Use <- {}", self.user));
                self.pass = rpassword::prompt_password(&format!("[password:{}]: ", self.user))
                    .unwrap_or_else(|_| String::new());
                println!();
            }
            "host" | "Host" => {
                self.host = value.to_string();
                print_suc(&format!("Host <- {}", self.host));
            }
            "pretty" | "Pretty" => {
                self.pretty = !self.pretty;
                print_suc(&format!("Pretty print <- {}", self.pretty));
            }
            "ns" | "NS" | "nameserver" | "namespace" => {
                self.namespace = value.to_string();
                print_suc(&format!("Namespace <- {}", self.namespace));
            }
            "db" | "DB" | "database" => {
                self.database = value.to_string();
                print_suc(&format!("Database <- {}", self.database));
            }
            "schema" | "Schema" | "sch" => {
                if value == "http" || value == "https" {
                    self.schema = value.to_string();
                    print_suc(&format!("Schema <- {}", self.schema));
                } else {
                    print_err("Invalid schema. Must be http or https.");
                }
            }
            _ => {
                print_err("No options.");
            }
        }
    }
}

/// Read password from stdin without echo
pub fn get_no_echo(prompt: &str) -> Result<String, anyhow::Error> {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout())?;
    let password = rpassword::read_password()?;
    println!();
    Ok(password)
}

/// Read input from stdin if piped
pub fn from_stdin() -> Option<String> {
    if atty::is(atty::Stream::Stdin) {
        return None;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input)
}
