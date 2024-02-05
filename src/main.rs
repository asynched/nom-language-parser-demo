use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space1},
    combinator::map,
    error::context,
    sequence::Tuple,
    IResult, Parser,
};

fn main() {
    let reader = BufReader::new(std::fs::File::open("commands.log").unwrap());
    let mut db = HashMap::<String, String>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (_, command) = parse_command(&line).unwrap();

        match command {
            Command::Set(key, value) => {
                db.insert(key, value);
                println!("OK");
            }
            Command::Get(key) => {
                if let Some(value) = db.get(&key) {
                    println!("{}", value);
                } else {
                    println!("nil");
                }
            }
            Command::Del(key) => {
                db.remove(&key);
                println!("OK");
            }
            Command::Incr(key) => {
                let value = db.entry(key.clone()).or_insert("0".to_string());
                let value = value.parse::<i64>().unwrap();
                db.insert(key, (value + 1).to_string());
                println!("OK");
            }
            Command::Flush => {
                db.clear();
                println!("OK");
            }
        }
    }
}

#[derive(Debug)]
enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Incr(String),
    Flush,
}

fn parse_command<'c>(command: &'c str) -> IResult<&'c str, Command, ()> {
    let (contents, command) = context("command", |command| {
        return (alt((
            context("parse-get", |command| {
                map(parse_get, |(_, _, key)| Command::Get(key.to_string())).parse(command)
            }),
            context("parse-set", |command| {
                map(parse_set, |(_, _, key, _, value)| {
                    Command::Set(key.to_string(), value.to_string())
                })
                .parse(command)
            }),
            context("parse-delete", |command| {
                map(parse_delete, |(_, _, key)| Command::Del(key.to_string())).parse(command)
            }),
            context("parse-flush", |command| {
                map(parse_flush, |_| Command::Flush).parse(command)
            }),
            context("parse-incr", |command| {
                map(parse_incr, |(_, _, key)| Command::Incr(key.to_string())).parse(command)
            }),
        )))
        .parse(command);
    })
    .parse(command)?;

    return Ok((contents, command));
}

fn parse_flush<'c>(command: &'c str) -> IResult<&'c str, &'c str, ()> {
    return (tag("FLUSH")).parse(command);
}

fn parse_delete<'c>(command: &'c str) -> IResult<&'c str, (&'c str, &'c str, &'c str), ()> {
    return (tag("DEL"), space1, alphanumeric1).parse(command);
}

fn parse_get<'c>(command: &'c str) -> IResult<&'c str, (&'c str, &'c str, &'c str), ()> {
    return (tag("GET"), space1, alphanumeric1).parse(command);
}

fn parse_set<'c>(
    command: &'c str,
) -> IResult<&'c str, (&'c str, &'c str, &'c str, &'c str, &'c str), ()> {
    return (tag("SET"), space1, alphanumeric1, space1, alphanumeric1).parse(command);
}

fn parse_incr<'c>(command: &'c str) -> IResult<&'c str, (&'c str, &'c str, &'c str), ()> {
    return (tag("INCR"), space1, alphanumeric1).parse(command);
}
