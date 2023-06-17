use std::{env, path::Path};

fn main() {
    
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

    println!("Hello, world!");

}
