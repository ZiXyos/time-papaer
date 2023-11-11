use back_time::{config::types::Parser, config::types::Setting, paper::types::Paper};
use std::{env, path::Path};
use wallpaper;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let path = Path::new(&args[1]);
        if path.exists() {
            let input = path.to_str().unwrap_or("");
            match path.with_extension("rc") == path {
                true => println!("files good"),
                false => {
                    eprintln!("file extention .rs not conventional please check help with -h");
                    return;
                }
            }

            let mut parser = Parser {
                input,
                items: vec![],
            };
            parser.parse_input().await;
            // println!("displaying {}", &path.display());
        } else {
            println!("");
        }
    }

    /* wallpaper::set_from_path("/Users/ktchoumh/dev/rust-projects/back-time/test/night-shift.jpg").unwrap();
    println!("{:?}", wallpaper::get());
    */
    /*let setting = Setting {
            weather: true,
        };
    */
    //let _runner = Paper::new(setting);
}
