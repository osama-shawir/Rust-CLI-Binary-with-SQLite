use rusqlite::{params, Connection, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "query")]
    Query,
    #[structopt(name = "insert")]
    Insert {
        ssn: String,
        income: i64,
    },
    #[structopt(name = "update")]
    Update {
        ssn: String,
        income: i64,
    },
    #[structopt(name = "delete")]
    Delete {
        ssn: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let conn = Connection::open("sql-murder-mystery.db")?;

    match args.cmd {
        Command::Query => {
            let mut stmt = conn.prepare("SELECT * FROM income")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
            })?;

            for row in rows {
                let (ssn, income) = row?;
                println!("SSN: {}, Income: {}", ssn, income);
            }
        }
        Command::Insert { ssn, income } => {
            conn.execute(
                "INSERT INTO income (ssn, annual_income) VALUES (?1, ?2)",
                params![ssn, income],
            )?;
        }
        Command::Update { ssn, income } => {
            conn.execute(
                "UPDATE income SET annual_income = ?1 WHERE SSN = ?2",
                params![income, ssn],
            )?;
        }
        Command::Delete { ssn } => {
            conn.execute("DELETE FROM income WHERE SSN = ?1", params![ssn])?;
        }
    }

    Ok(())
}