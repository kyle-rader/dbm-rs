use clap::Parser;
use postgres::{Client, NoTls};
mod env_file;

#[derive(Debug, Parser)]
struct Args {
    /// Enable verbose output
    #[clap(long)]
    verbose: bool,

    /// Database connection URL
    #[clap(long, env = "DATABASE_URL")]
    db_url: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Args { verbose, db_url } = Args::parse();
    println!("Verbose mode is {}", if verbose { "on" } else { "off" });

    let db_url = db_url.unwrap_or_else(|| {
        let env_file = env_file::EnvFile::load();
        if let Ok(env) = env_file {
            if let Some(db_url) = env.database_url {
                return db_url;
            }
        }

        eprintln!("error: The --db-url or DATABASE_URL env var (from environment or .env file) must be set");
        std::process::exit(1);
    });

    println!("Database URL is {}", db_url);

    let mut client = Client::connect(&db_url, NoTls)?;

    let dbs = client.query(
        "SELECT datname FROM pg_database WHERE datistemplate = false;",
        &[],
    )?;

    let dbs = dbs
        .into_iter()
        .map(|row| row.get(0))
        .collect::<Vec<String>>();
    println!("Databases: {dbs:#?}");

    Ok(())
}
