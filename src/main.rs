use chrono::Datelike;
use clap::{App, Arg, SubCommand};

//mod solutions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let matches = App::new("Advent of Code 2021 Solutions")
        .subcommand(SubCommand::with_name("download").arg(Arg::with_name("day")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("download") {
        download(matches.value_of("day")).await?;
    }

    Ok(())

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     panic!("Need to specify day")
    // }
    // let day: u8 = args[1].parse().unwrap();
    // let parts: u8 = if args.len() > 2 {
    //     args[2].parse().unwrap()
    // } else {
    //     3
    // };
    // solutions::solve(day, parts);
}

// fn flatMapInner(x: String) -> Vec<String> {
//     return x.split(',').map(|x| String::from(x)).collect();
// }

// .flat_map(|x| {
//     //x.split(',')
//     x.chars().map(|x| String::from(x)).collect::<Vec<String>>()
// })
// for token in res {
//     println!("token: {} ({})", token, token.len());
// }

async fn download(day: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let default_day = chrono::Utc::now().day().to_string();
    let selected_day = day.unwrap_or(&default_day);

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
            "https://adventofcode.com/2020/day/{}/input",
            selected_day
        ))
        .send()
        .await?
        .text()
        .await?;

    std::fs::write(format!("inputs/{}", selected_day), body)?;

    Ok(())
}
