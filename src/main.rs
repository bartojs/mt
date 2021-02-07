use std::process;
use std::path::Path;
use std::env;
use std::fs;
use chrono::prelude::{DateTime, Local};

fn main() {
    for arg in env::args().skip(1) {
        let p = Path::new(&arg);
        if p.exists() && p.is_file() {
            let f = p.canonicalize().unwrap();
            let d = fs::metadata(&f).unwrap();
            let m = d.modified().unwrap();
            let s = m.elapsed().unwrap().as_secs();
            if s < 60 {
                println!("{} ({}) {}secs", f.display(), arg, s);
            } else if s < 60*60 {
                println!("{} ({}) {}mins", f.display(), arg, s/60);
            } else if s < 60*60*60 {
                println!("{} ({}) {}hrs", f.display(), arg, s/(60*60));
            } else {
                let dt: DateTime<Local> = m.into();
                println!("{} ({}) {}", f.display(), arg, dt.format("%F|%T"));
            }
        } else {
            println!("{} - no file", arg);
            process::exit(1);
        }
    }
    process::exit(0);
}
