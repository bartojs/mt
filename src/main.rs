use std::process;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    for arg in env::args().skip(1) {
        let p = Path::new(&arg);
        if p.exists() && p.is_file() {
            let f = p.canonicalize().unwrap();
            let m = fs::metadata(&f).unwrap();
            println!("{} ({}) {:?} secs", f.display(), arg, m.modified().unwrap().elapsed().unwrap().as_secs());
        } else {
            println!("{} - no file", arg);
            process::exit(1);
        }
    }
    process::exit(0);
}
