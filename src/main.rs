extern crate num;

use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use num::pow;

fn rabin_karp(text: &[u8], patt_list: &[&[u8]]) -> Result<Vec<Vec<i32>>, String> {
    let m = patt_list.into_iter().map(|arr| arr.len()).max().unwrap();
    let n = text.len();
    let l = patt_list.len();

    if m != patt_list.into_iter().map(|arr| arr.len()).min().unwrap() {
        return Err("The lengths of patterns have to be the same. ".to_string());
    }

    if n < m {
        return Err("The length of input text have to be longer than that of pattern. ".to_string());
    }

    const BASE: u64 = 7;

    // init hashes
    let mut ht: u64 = 0;
    for i in 0..m { // O(m)
        debug_assert!(i < m);
        ht = ht * BASE + (text[i] as u64);
    }

    let mut hp_list: HashMap<u64, Vec<usize>> = HashMap::new();
    for (i, arr) in patt_list.into_iter().enumerate() { // O(l)
        let h = arr.into_iter().scan(0, |state, &x| {
            *state = *state * BASE + (x as u64);
            Some(*state)
        }).last().unwrap();

        let exist = hp_list.contains_key(&h);
        if exist {
            hp_list.get_mut(&h).unwrap().push(i);
        } else {
            let mut v = Vec::new();
            v.push(i);
            hp_list.insert(h, v);
        }
    }
   

    // prepare a result container
    let mut result: Vec<Vec<i32>> = Vec::new();
    for _ in 0..l { // O(l)
        result.push(Vec::new());
    }

    
    // search
    for i in 0..n-m+1 {
        // compare
        match hp_list.get(&ht) { // O(n-m)
            Some(v) => {
                for &j in v {
                    let mut flag = true;
                    let patt: &[u8] = patt_list[j];
                    for k in 0..m { // O((n-m)m)
                        debug_assert!(i+k < n);
                        debug_assert!(k < m);
                        flag = flag & (text[i+k] == (patt[k] as u8));
                    }
                    if flag {
                        result[j].push(i as i32);
                    }
                }
            }
            None => {}
        }

        if i == n-m { // O(n)
            break;
        }
        
        // calculate next hash O(n)
        debug_assert!(i+m < n);
        ht = (ht - (text[i] as u64) * pow(BASE, m-1 as usize)) * BASE + (text[i+m] as u64);
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage : cargo run text_filename pattern1 pattern2 ...");
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


    let patterns = &args[2..args.len()];
    let patt_list = patterns.into_iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();

    match rabin_karp(text.as_bytes(), &patt_list) {
        Ok(v) => {
            for (i, arr) in v.into_iter().enumerate() {
                print!("{} : ", patterns[i as usize]);
                for x in &arr {
                    print!("{}, ", x);
                }
                println!("");
            }
        }
        Err(msg)    => println!("{}", msg),
    }
}
