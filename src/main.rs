extern crate rustc_serialize;
extern crate docopt;
extern crate postgres;
extern crate rand;
extern crate chrono;

use docopt::Docopt;
use postgres::{Connection, SslMode};
use rand::{thread_rng, Rng};
use chrono::{UTC, Duration};

static USAGE: &'static str = "
Usage:
    pgrand <command> <table> [--size=<size>]
    pgrand (-h | --help)
    pgrand (-v | --version)

Options:
    -h --help       Show this screen.
    -v --version    Show version. 
    --size=<size>   number of rows to insert into table.

Some pgrand commands are:
    create  Creates a new table with random rows.
    drop    Drops the specified table.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_command: Command,
    arg_table: String,
    flag_size: Option<usize>
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Create, Drop
}

#[allow(dead_code)]
struct Table {
    id: i32,
    message: String
}


fn create(conn: &Connection, table: &str, size: usize) {
    drop(conn, table);

    conn.execute(&format!("CREATE TABLE {} (
                    id SERIAL PRIMARY KEY,
                    message VARCHAR NOT NULL,
                    time TIMESTAMP WITH TIME ZONE)", table), &[]).unwrap();
    
    let dt = UTC::now();
    for i in 0..size {
        let new_time = dt + Duration::seconds((100*i) as i64);
        let msg: String = thread_rng().gen_ascii_chars().take(10).collect();
        conn.execute(&format!("INSERT INTO {}(message, time) VALUES($1, $2)", table),
        &[&msg, &new_time]).unwrap();
    }
}

fn drop(conn: &Connection, table: &str) {
    conn.execute(&format!("DROP TABLE IF EXISTS {}", table), &[]).unwrap();
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let conn = Connection::connect("postgres://tal@localhost", &SslMode::None).unwrap();

    match args.arg_command {
        Command::Create => create(&conn, &args.arg_table, args.flag_size.unwrap_or_else(|| 1000)),
        Command::Drop => drop(&conn, &args.arg_table)
    }
}
