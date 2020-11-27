mod solutions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Need to specify day")
    }
    let day: u8 = args[1].parse().unwrap();
    let parts: u8 = if args.len() > 2 {
        args[2].parse().unwrap()
    } else {
        3
    };
    solutions::solve(day, parts);
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
