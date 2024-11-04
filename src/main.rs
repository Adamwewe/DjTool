// use std::env::args;
// use audiotags::{AnyTag, AudioTagEdit, Id3v2Tag}; // audio conversion crate
// use glob::glob;
mod folder_crawler;
mod parse_path;
mod encoder;

use folder_crawler::Crawler;
use parse_path::FolderParser;


use crate::encoder::parseToBytes;
use crate::encoder::EncodeTrack;



fn main() {

    let parsed = FolderParser::parser();
    let items = Crawler::new(parsed.path)
        .crawl();
    
    items.iter()
    .map(|x| EncodeTrack(&x))
    .collect::<Vec<_>>();

/*

steps:
    - figure out if flac or wav
    
    if flac:
        - Read flac Header
            - Parse metadata blocks
            - Decode audio frames
            - subframe decoding
            - Apply residual encoding
            - 
 */


    // let bytes : Option<Vec<u8>> = parseToBytes(items);


//     let encoded  : Vec<u8> = pcmEncoder(bytes.unwrap());

//     println!("{:?}", encoded);
}
