use clap::Parser;
use homedir::get_my_home;
use rusqlite::{params, Connection, Result};
use std::env;
use std::env::set_current_dir;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct CliArguments {
    action: String,
    alias: Option<String>,
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let mut db_path: PathBuf = get_my_home().unwrap().unwrap();
    db_path.push("fs_bookmark/fs_bookmark.db");
    println!("{}", db_path.display());
    let con = Connection::open((db_path))?;
    con.execute(
        "create table if not exists fs_bookmark (
            alias TEXT PRIMARY KEY,
        path TEXT NOT NULL
    )",
        (),
    )?;

    let args: CliArguments = CliArguments::parse();

    let path: PathBuf = match args.path {
        Some(p) => p,
        None => env::current_dir().expect(
            "Nor you have entered a path, neither the working directory can be figured out.",
        ),
    };

    if args.action == "create" {
        create(&args.alias, &path, &con)?;
    } else if args.action == "jump" {
        jump(&args.alias, &con)?;
    } else if args.action == "remove" {
        remove_bookmark(&args.alias, &con)?;
    } else if args.action == "list" {
        list_bookmarks( &con)?;
    }

    println!(
        "action: {:?}, alias: {:?}, path: {:?}",
        args.action, args.alias, path
    );

    Ok(())
}

// TODO: Add bookmark

fn create(alias: &String, path: &PathBuf, con: &Connection) -> Result<()> {
    con.execute(
        "INSERT INTO fs_bookmark VALUES(?1, ?2)",
        params![alias, path.to_str()],
    )?;
    Ok(())
}

// TODO: Remove bookmark

fn remove_bookmark(alias: &String, con: &Connection) -> Result<()> {
    con.execute("DELETE FROM fs_bookmark WHERE alias=(?1)", params![alias])?;
    Ok(())
}

// TODO: Jump to

fn jump(alias: &String, con: &Connection) -> Result<()> {
    let new_workdir = con.execute("SELECT path FROM fs_bookmark WHERE alias='?1';", params![alias]);
    // assert!(env::set_current_dir(Some(&new_workdir))
    // .is_ok());
    Ok(())
}


fn list_bookmarks( con: &Connection) -> Result<()> {
    let bookmark_list = con.execute("SELECT * FROM fs_bookmark;", ());

    println!("{:?}", bookmark_list);

    Ok(())
}

// TODO: Add help menu

fn help() {}

