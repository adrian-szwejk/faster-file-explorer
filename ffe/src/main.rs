//use std::env;
//use std::fs;
//use std::io;

extern crate glob;
//use glob::glob;
extern crate walkdir;
use walkdir::{DirEntry, WalkDir};
use std::path::{Path, PathBuf};

extern crate directories;
use directories::{BaseDirs, UserDirs, ProjectDirs};
                                                                   
fn main() {
    // let x = 3;
    // let y = 4;
    // println!("My favorite number is {} + {} = {}", x, y, x + y);
    // let args: Vec<String> = env::args().collect();
    // //dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");
    // println!("With text:\n{contents}");
    
    //Used to chose directory within project
    // let paths = fs::read_dir("../../").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    // for e in glob("../*").expect("Failed to read glob pattern") {
    //     match e {
    //         Ok(path) => println!("{:?}", path.display()),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }

    
    println!("UserDirs:");
    let music_path = get_music_path();
    
    WalkDir::new(music_path)
    .min_depth(0)
    .max_depth(1)
    .into_iter()
    //.filter_entry(|e| is_not_hidden(e))
    .filter_map(|v| v.ok())
    .for_each(|x| println!("{}", x.path().display()));
    

    let c:PathBuf = get_base_path();
    println!("BaseDirs:");
    // for i in WalkDir::new(c).into_iter().filter_map(|e| e.ok()) {
    //     if i.metadata().unwrap().is_file() {
    //         println!("{}", i.path().display());
    //     }
    // }
                                                                  
    //Used to find global directories
}

//cargo run -- test test_doc.txt


fn get_music_path() -> PathBuf{
    let mut music_path= PathBuf::new(); 
            
    if let Some(user_dirs) = UserDirs::new() {
        //println!("{}", user_dirs.home_dir().display());
        music_path = user_dirs.home_dir().to_path_buf();
    }
    music_path
}

fn get_base_path() -> PathBuf{
    let mut c_drive= PathBuf::new(); 
    if let Some(base_dirs) = BaseDirs::new() {
        c_drive=base_dirs.home_dir().to_path_buf();                                             
    }
    c_drive
}

//Function to get rid of hidden files directories
//TODO: This also gets rid of ./ folder directories
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| entry.depth() == 0 || !s.starts_with("."))
         .unwrap_or(false)
}