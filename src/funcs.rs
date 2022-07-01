use std::{io::*, fs::*, path::*};

pub fn openfile<S: AsRef<Path> + ?Sized>(s: &S) -> (Vec<u8>, usize) {
    let mut file = File::open(s).unwrap();
    let mut vec: Vec<u8> = Vec::new();
    file.read_to_end(&mut vec).unwrap();
    let s = vec.len();
    let p = s - 4;
    return (vec, p);
}

pub fn writepairtofile<S: AsRef<Path> + ?Sized>(s: &S, pair: (Vec<u8>, usize)) {
    let (buf, p) = pair;
    let arr = &buf[p..];
    let mut writer = File::create(s).unwrap();
    writer.write(arr).unwrap();
}

pub fn overwritefile<S1: AsRef<Path> + ?Sized, S2: AsRef<Path> + ?Sized>(og: &S1, hex: &S2) {
    let mut reader = File::open(og).unwrap();
    let mut ogbuf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut ogbuf).unwrap();
    drop(reader);
    reader = File::open(hex).unwrap();
    let mut hexbuf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut hexbuf).unwrap();
    drop(reader);
    let s = ogbuf.len();
    let p = s - 4;
    let mut idx = 0;
    for i in p..ogbuf.len() {
        ogbuf[i] = hexbuf[idx];
        idx = idx + 1;
    }
    let mut file = OpenOptions::new().write(true).truncate(true).open(og).unwrap();
    file.write_all(&mut ogbuf).unwrap();
}