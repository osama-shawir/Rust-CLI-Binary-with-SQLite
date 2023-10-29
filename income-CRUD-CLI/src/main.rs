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
            let mut stmt = conn.prepare("SELECT ssn, annual_income FROM income
            ORDER BY annual_income DESC
            LIMIT 10")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
            })?;

            for row in rows {
                let (ssn, income) = row?;
                println!("SSN: {}, Income: {}", ssn, income);
            }
        }

        Command::Insert { ssn, income } => {
            let rows_affected = conn.execute(
                "INSERT INTO income (ssn, annual_income) VALUES (?1, ?2)",
                params![ssn, income],
            )?;

            if rows_affected == 1 {
                println!("Insert operation completed successfully");
            } else {
                println!("Insert operation did not affect any rows");
            }
        }

        Command::Update { ssn, income } => {
            let rows_affected = conn.execute(
                "UPDATE income SET annual_income = ?1 WHERE SSN = ?2",
                params![income, ssn],
            )?;

            if rows_affected == 1 {
                println!("Update operation completed successfully");
            } else {
                println!("Update operation did not affect any rows");
            }
        }

        Command::Delete { ssn } => {
            let rows_affected = conn.execute("DELETE FROM income WHERE SSN = ?1", params![ssn])?;

            if rows_affected == 1 {
                println!("Delete operation completed successfully");
            } else {
                println!("Delete operation did not affect any rows");
            }
        }
    }

    Ok(())
}