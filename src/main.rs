extern crate num;

use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use num::pow;

fn rabin_karp(text: &[u8], patt: &[u8]) -> Option<i32> {
    let m = patt.len();
    let n = text.len();

    if n < m {
        return None;
    }

    let base: u64 = 7;

    let mut ht: u64 = 0;
    let mut hp: u64 = 0;

    // init hashes
    for i in 0..m { // O(m)
        debug_assert!(i < m);
        ht = ht * base + (text[i] as u64);
        hp = hp * base + (patt[i] as u64);
    }

    for i in 0..n-m+1 {
        // check
        if ht == hp { // O(n)
            let mut flag = true;
            for k in 0..m { // O(mn)
                debug_assert!(i+k < n);
                debug_assert!(k < m);
                flag = flag & (text[i+k] == patt[k]);
            }
            if flag {
                return Some(i as i32);
            }
        }

        if i == n-m { // O(n)
            break;
        }
        
        // calculate next hash O(n)
        debug_assert!(i+m < n);
        ht = (ht - (text[i] as u64) * pow(base, m-1 as usize)) * base + (text[i+m] as u64);
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage : cargo run text_filename pattern_filename");
    }

    // read text file
    let path = Path::new(&args[1]);
    let mut reader = BufReader::new(
        match File::open(&path) {
            Err(why)    => panic!("Could not open a file. :{}", Error::description(&why)),
            Ok(file)    => file,
        });
    let mut text = String::new();
    let _ = reader.read_line(&mut text);
    let text = text;

    // read pattern file
    let path = Path::new(&args[2]);
    let mut reader = BufReader::new(
        match File::open(&path) {
            Err(why)    => panic!("Could not open a file. :{}", Error::description(&why)),
            Ok(file)    => file,
        });
    let mut pattern = String::new();
    let _ = reader.read_line(&mut pattern);
    let pattern = &pattern.trim_right();
    println!("Pattern : \"{}\"", pattern);

    match rabin_karp(text.as_bytes(), pattern.as_bytes()) {
        Some(n) => println!("Found : {}", n),
        None    => println!("Not Found")
    }
}
