//use std::env;
use std::fs;
use std::io;

extern crate glob;
use glob::glob;
extern crate walkdir;
use walkdir::WalkDir;
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

    
    println!("BaseDirs:");
    let music_path = get_music_path();
 
    for i in WalkDir::new(music_path).into_iter().filter_map(|e| e.ok()) {
        if i.metadata().unwrap().is_file() {
            println!("{}", i.path().display());
        }
    }
                                                                  
    //Used to find global directories
}

//cargo run -- test test_doc.txt

fn get_music_path() -> PathBuf{
    let mut music_path= PathBuf::new(); 
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(i) = user_dirs.audio_dir() {
            //println!("{:?}",i);
            music_path=i.to_path_buf();
        }    
        // Lin: /home/alice/Music
        // Win: C:\Users\Alice\Music
        // Mac: /Users/Alice/Music
    }
    music_path
}
