use std::{env, path::Path};
use back_time::{paper::types::Paper, config::types::Setting, config::types::Parser};
use wallpaper;


#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() >= 2 {

        let path = Path::new(&args[1]);
        if path.exists() {
            let input = path.to_str().unwrap_or("");
            let mut parser = Parser {
                input,
                items: vec![]
            };

            parser.parse_input().await;
            println!("{}", &path.display());
        } else {
            println!("");
        }
    }

    wallpaper::set_from_path("/Users/ktchoumh/dev/rust-projects/back-time/test/night-shift.jpg").unwrap();
    println!("{:?}", wallpaper::get());

    /*let setting = Setting {
        weather: true,
    };
*/
    //let _runner = Paper::new(setting);
    
}
