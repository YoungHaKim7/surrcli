use crate::models::SurrDB;
use anyhow::Result;

impl SurrDB {
    /// Generate Basic Auth header value
    pub fn basic_auth(&self) -> String {
        let auth = format!("{}:{}", self.user, self.pass);
        use base64::prelude::*;
        BASE64_STANDARD.encode(auth)
    }

    /// Send SQL query to SurrealDB
    pub fn requester(&self, query: &str) -> Result<(String, u16)> {
        let url = format!("{}://{}/sql", self.schema, self.host);

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(self.timeout))
            .build()?;

        let response = client
            .post(&url)
            .header("Authorization", format!("Basic {}", self.basic_auth()))
            .header("NS", &self.namespace)
            .header("DB", &self.database)
            .header("Accept", "application/json")
            .body(query.to_string())
            .send()?;

        let status = response.status().as_u16();
        let body = response.text()?;

        Ok((body, status))
    }

    /// Execute query and print result
    pub fn contact_surr(&self, query: &str) -> Result<()> {
        let (resp, _) = self.requester(query)?;

        if self.pretty {
            crate::utils::pretty_print(&resp)?;
        } else {
            crate::utils::print_raw(&resp);
        }

        Ok(())
    }

    /// Test connection to SurrealDB
    pub fn test_connection(&self) -> Result<()> {
        let (_, status) = self.requester("INFO FOR DB;")?;

        match status {
            200 => crate::utils::print_suc("Connection is OK!"),
            403 => {
                crate::utils::print_err(
                    "There was a problem with authentication.\nUse .set user <username> to reset credentials."
                );
            }
            _ => {
                crate::utils::print_err("Error!");
            }
        }

        Ok(())
    }
}
