use clap::Parser;
use surrcli::db::db_file_init;
use surrcli::models::SurrDB;
use surrcli::utils::from_stdin;

/// SurrealCLI - Client command line tool for managing SurrealDB
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host address (IP:PORT) without schema
    #[arg(short = 'h', long, default_value = "127.0.0.1:8000")]
    host: String,

    /// Username
    #[arg(short, long, default_value = "root")]
    user: String,

    /// Password (will prompt if not provided)
    #[arg(short, long, default_value = "hide")]
    pass: String,

    /// Database name
    #[arg(short = 'D', long, default_value = "surr")]
    database: String,

    /// Namespace
    #[arg(short = 'N', long, default_value = "surr")]
    namespace: String,

    /// Schema (http or https)
    #[arg(short = 's', long, default_value = "http")]
    schema: String,

    /// Run query directly
    #[arg(short, long, default_value = "none")]
    query: String,

    /// Use existing profile
    #[arg(long, default_value = "none")]
    profile: String,

    /// Connection timeout in seconds
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,

    /// Pretty print JSON output
    #[arg(long, default_value_t = true)]
    pretty: bool,

    /// Number of completion suggestions (0 to disable)
    #[arg(short, long, default_value_t = 5)]
    comple: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Get password
    let password = if args.pass == "hide" {
        surrcli::utils::get_no_echo("[password]: ")?
    } else {
        println!();
        args.pass.clone()
    };

    // Initialize SurrDB configuration
    let mut surrdb = SurrDB {
        host: args.host.clone(),
        namespace: args.namespace.clone(),
        database: args.database.clone(),
        user: args.user.clone(),
        pass: password,
        schema: args.schema.clone(),
        pretty: args.pretty,
        timeout: args.timeout,
        comple: args.comple,
        query: String::new(),
    };

    // Initialize database
    db_file_init()?;

    // Load profile if specified
    if args.profile != "none" {
        surrdb.db_set_profile_by_idx(&args.profile)?;
    }

    // Check for stdin input
    if let Some(stdin_query) = from_stdin() {
        surrdb.contact_surr(stdin_query.trim())?;
        return Ok(());
    }

    // Either run query directly or enter interactive mode
    if args.query == "none" {
        surrdb.init_cli()?;
    } else {
        surrdb.contact_surr(&args.query)?;
    }

    Ok(())
}
