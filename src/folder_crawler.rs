use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Crawler{
    pub folders : HashMap<String, Vec<String>>,
    pub path : String
}

impl Crawler {
    pub fn new(path : String) -> Self {
        Self{folders: HashMap::new(),
            path: path}
    }

    pub fn crawl(&mut self) {//-> Option<ReadDir> {
    
        let items = fs::read_dir(String::from("./")
             + &self.path.clone().to_owned())
            .ok(); // cheap cloning

            for item in items.unwrap() {
                println!("Name: {}", &item.unwrap().path().display());
            }

    // pub fn view(&self) {
        //https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    // }
    }
}