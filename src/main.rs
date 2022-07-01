mod funcs;
use std::{*, path::*};
use funcs::*;

fn main() {
    let envargs: Vec<String> = env::args().collect();
    let args = &envargs[1..];
    for arg in args {
        let path = Path::new(arg);
        let stem = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let hexname = stem + ".hex";
        let hexfile = Path::new(&hexname);
        if hexfile.exists() {
            overwritefile(arg, hexfile);
        } else {
            let pair = openfile(arg);
            writepairtofile(hexfile, pair);
        }
    }
}
