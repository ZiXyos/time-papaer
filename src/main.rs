use std::{env, path::Path};
use back_time::{paper::types::Paper, config::types::Setting};
use wallpaper;

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() >= 2 {

        let path = Path::new(&args[1]);
        if path.exists() {
            println!("{}", &path.display());
        } else {
            println!("");
        }
    }

    wallpaper::set_from_path("/Users/ktchoumh/dev/rust-projects/back-time/test/noon.jpg").unwrap();
    println!("{:?}", wallpaper::get());

    let setting = Setting {
        weather: true,
        time: true,
        ambiant: true,
        source: String::from("./"),
        sources: Vec::from([String::from("./")]),
        resize_mode: true,
        width: 1920,
        height: 1080,
    };

    let runner =  Paper::new(setting);
    runner.hourly_change(String::from("./")).await;
}
