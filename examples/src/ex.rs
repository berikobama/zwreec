use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashMap;


fn main() {
	let args: Vec<String> = env::args().collect();
	let pathname: &str;
	if args.len() == 2 {
		pathname = &args[1];
	} else {
		println!("no input file");
		return;
	}
	let content = read_file(pathname);
	println!("Inhalt: \n{}",content);
	let hmap = count_chars(&content);
	let mut tmp = String::new();
	for (k, l) in hmap.iter() {
		let t = k.to_string()+":"+&l.to_string()+"\n";
		tmp.push_str(&t);
	}
	write_file("asdf.txt",&tmp)
}

fn read_file(pathname:&str)-> String{
    let path = Path::new(pathname);
    let display = path.display();
    let mut file = match File::open(&path) { //read only
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => s
    }
}

fn write_file(pathname: &str,content: &str){
	let path = Path::new(pathname);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn count_chars(content: &str)->HashMap<char, i32>{
	let mut h = HashMap::new();
	for a in content.chars(){
		if h.contains_key(&a){
			let k:i32;
			match h.get_mut(&a){
				Some(i) => k = *i,
				None => k = -1,
			};
			h.insert(a,k+1);
		} else {
			h.insert(a,1);
		}
	}
	h
}