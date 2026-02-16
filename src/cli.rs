use crate::db::{db_drop_idx, db_drop_query_idx, db_get_query_by_idx, db_show_profiles, db_show_queries};
use crate::meta::{banner, help};
use crate::models::SurrDB;
use crate::utils::print_err;
use anyhow::Result;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Editor, Helper};
use std::borrow::Cow;

struct SurrCompleter;

impl Helper for SurrCompleter {}

impl Completer for SurrCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<String>), ReadlineError> {
        let suggestions = vec![
            "SELECT".to_string(),
            "FROM".to_string(),
            "WHERE".to_string(),
            "LIMIT".to_string(),
            "CREATE".to_string(),
            "RELATE".to_string(),
            "CONTENT".to_string(),
            "INFO".to_string(),
            "FOR".to_string(),
            "DB".to_string(),
            "NS".to_string(),
            "TABLE".to_string(),
            "GROUP".to_string(),
            "BY".to_string(),
            ".help".to_string(),
            ".options".to_string(),
            ".set".to_string(),
            ".save".to_string(),
            ".show".to_string(),
            ".delete".to_string(),
            ".run".to_string(),
            "query".to_string(),
            "profile".to_string(),
        ];

        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let current = &line[start..pos];

        let matches: Vec<String> = suggestions
            .iter()
            .filter(|s| s.starts_with(current))
            .cloned()
            .collect();

        Ok((start, matches))
    }
}

impl Hinter for SurrCompleter {
    type Hint = String;
}

impl Highlighter for SurrCompleter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        false
    }
}

impl Validator for SurrCompleter {}

impl SurrDB {
    /// Initialize interactive CLI
    pub fn init_cli(&mut self) -> Result<()> {
        banner();
        self.test_connection()?;
        println!();

        let mut rl = Editor::new().unwrap();
        rl.set_helper(Some(SurrCompleter));

        let prompt = format!("[{}]> ", self.database);

        loop {
            let readline = rl.readline(&prompt);
            match readline {
                Ok(line) => {
                    let _ = rl.add_history_entry(line.as_str());
                    self.execute(&line)?;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Execute a command from the CLI
    pub fn execute(&mut self, input: &str) -> Result<()> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match input {
            ".help" => help(),
            ".options" => self.show_options(),
            _ => {
                if parts.is_empty() {
                    return Ok(());
                }

                match parts[0] {
                    ".set" => {
                        if parts.len() == 3 {
                            self.set_var(parts[1], parts[2]);
                        } else {
                            print_err("Usage: .set <variable> <value>");
                        }
                    }
                    ".save" => self.save_commands(&parts),
                    ".delete" => self.delete_commands(&parts),
                    ".show" => self.show_commands(&parts),
                    ".run" => self.run_commands(&parts),
                    _ => {
                        self.query = input.to_string();
                        self.contact_surr(input)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn save_commands(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            print_err("Usage: .save profile|query <name>");
            return;
        }

        match parts[1] {
            "profile" => {
                if parts.len() != 3 {
                    print_err("Usage: .save profile <name>");
                    return;
                }
                let _ = self.db_save_profile(parts[2]);
            }
            "query" => {
                if parts.len() != 3 {
                    print_err("Usage: .save query <name>");
                    return;
                }
                let _ = self.db_save_query(parts[2]);
            }
            _ => {
                print_err("Not a command. Use: profile|query");
            }
        }
    }

    fn delete_commands(&self, parts: &[&str]) {
        if parts.len() < 2 {
            print_err("Usage: .delete profile|query <name>");
            return;
        }

        match parts[1] {
            "profile" => {
                if parts.len() != 3 {
                    print_err("Usage: .delete profile <name>");
                    return;
                }
                let _ = db_drop_idx(parts[2]);
            }
            "query" => {
                if parts.len() != 3 {
                    print_err("Usage: .delete query <name>");
                    return;
                }
                let _ = db_drop_query_idx(parts[2]);
            }
            _ => {
                print_err("Not a command. Use: profile|query");
            }
        }
    }

    fn show_commands(&self, parts: &[&str]) {
        if parts.len() != 2 {
            print_err("Usage: .show profiles|queries");
            return;
        }

        match parts[1] {
            "profiles" => {
                let _ = db_show_profiles();
            }
            "queries" => {
                let _ = db_show_queries();
            }
            _ => {
                print_err("Not a command. Use: profiles|queries");
            }
        }
    }

    fn run_commands(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            print_err("Usage: .run profile|query <name>");
            return;
        }

        match parts[1] {
            "profile" => {
                if parts.len() != 3 {
                    print_err("Usage: .run profile <name>");
                    return;
                }
                let _ = self.db_set_profile_by_idx(parts[2]);
            }
            "query" => {
                if parts.len() != 3 {
                    print_err("Usage: .run query <name>");
                    return;
                }
                match db_get_query_by_idx(parts[2]) {
                    Ok(query) => {
                        crate::utils::print_suc("Running query");
                        let _ = self.contact_surr(&query);
                    }
                    Err(_) => {
                        print_err("Error searching query name.");
                    }
                }
            }
            _ => {
                print_err("Not a command. Use: profile|query");
            }
        }
    }
}
