use chrono::Datelike;
use clap::{App, Arg, SubCommand};
use std::io::Read;

mod solutions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let matches = App::new("Advent of Code 2021 Solutions")
        .arg(Arg::with_name("part"))
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("download").arg(Arg::with_name("day")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("download") {
        return download(get_day(matches.value_of("day"))).await;
    }

    let part: isize = matches.value_of("part").unwrap_or("3").parse()?;
    let day = get_day(matches.value_of("day"));
    solutions::solve(day, part);

    Ok(())
}

async fn download(selected_day: isize) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    let token = std::env::var("TOKEN");
    if let Err(_) = token {
        panic!("Missing TOKEN env variable");
    }
    let cookie = format!("session={}", token?);
    headers.insert("cookie", reqwest::header::HeaderValue::from_str(&cookie)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let body = client
        .get(format!(
            "https://adventofcode.com/2021/day/{}/input",
            selected_day
        ))
        .send()
        .await?
        .text()
        .await?;

    if body.contains("Please don't repeatedly request this endpoint") {
        println!("Day {:02} not yet ready", selected_day);
        return Ok(());
    }

    std::fs::write(format!("inputs/{:02}", selected_day), body)?;

    copy_template(selected_day)?;

    append_day(selected_day)?;

    Ok(())
}

fn copy_template(selected_day: isize) -> Result<u64, std::io::Error> {
    return std::fs::copy(
        "src/solutions/template.rs",
        format!("src/solutions/day{:02}.rs", selected_day),
    );
}

fn append_day(selected_day: isize) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("src/solutions/mod.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let new_mod = format!("mod day{:02};", selected_day);
    contents = contents.replace("mod solver;", &format!("{}\nmod solver;", new_mod));

    let new_match = format!(
        "{} => day{:02}::Problem.solve(filename, parts),",
        selected_day, selected_day
    );
    contents = contents.replace(
        "_ => panic!",
        &format!("{}\n        _ => panic!", new_match),
    );

    return std::fs::write("src/solutions/mod.rs", contents);
}

fn get_day(day: Option<&str>) -> isize {
    let default_day = chrono::Utc::now().day().to_string();
    day.unwrap_or(&default_day).parse().unwrap()
}
