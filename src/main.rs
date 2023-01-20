// use std::env::args;
// use audiotags::{AnyTag, AudioTagEdit, Id3v2Tag}; // audio conversion crate
// use glob::glob;
mod folder_crawler;
mod parse_path;

use folder_crawler::Crawler;
use parse_path::FolderParser;

mod converter;
use converter::parser;

fn main() {

    let parsed = FolderParser::parser();
    let items = Crawler::new(parsed.path)
        .crawl();
    // println!("{:?}",items);

    parser(items);
}
