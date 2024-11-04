
use std::fs::File;
use std::io::{Read, BufReader};
use std::fmt::Debug;

use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::probe::Hint;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};

use id3::{Tag, TagLike};


pub fn EncodeTrack(path : &str) -> Vec<i32> {//Result<(Vec<f32>, u32), Box<dyn std::error::Error>> {
    let vec = vec![1];
    let src = std::fs::File::open(path).expect("failed to open media");
    // let src : ReadOnlySource<R> = ReadOnlySource::new(path.into());//.expect("failed to open media");

    // Get ID3v2 tags
    let tag = Tag::read_from_path(path);
    
    // let input_stream = MediaSourceStream::from_path(path)?;
    let mss = MediaSourceStream::new(Box::new(src), 
                            Default::default());
     // Create the media source stream.

     // Use the default options for metadata and format readers.
     let meta_opts: MetadataOptions = Default::default();
     let fmt_opts: FormatOptions = Default::default();
    
     let mut hint = Hint::new();
     hint.with_extension("mp3");


    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();
    
    println!("{:?}, {:?}, {:?}", format_opts, metadata_opts, decoder_opts);

    // Probe the media source stream for a format.
    // PARSING ERRORS!!!
    let probed = symphonia::default::get_probe()
                .format(&hint, mss, &format_opts, &metadata_opts)
                .expect("unsupported format");

    let mut format = probed.format;
    println!("{:?}", path);

    // Get the default track.
    let track = format
        .default_track();

    // Create a decoder for the track.
    // let mut decoder =
    //     symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
    // // Get the instantiated format reader.
    // let mut format = probed.format;

    // // Find the first audio track with a known (decodeable) codec.
    // let track = format
    //     .tracks()
    //     .iter()
    //     .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
    //     .expect("no supported audio tracks");

    // // Use the default options for the decoder.
    // let dec_opts: DecoderOptions = Default::default();

    // // Create a decoder for the track.
    // let mut decoder = symphonia::default::get_codecs()
    //     .make(&track.codec_params, &dec_opts)
    //     .expect("unsupported codec");

    // // Store the track identifier, it will be used to filter packets.
    // let track_id = track.id;

    // println!("{:?}", track_id);

    vec
}

pub fn parseToBytes(path_vec: Vec<String>) -> Option<Vec<u8>>{

    // Refactor to remove all unwraps

    // Retrieving from file signature
    let mut audio_vecs : Vec<u8> = Vec::new();
    let mut header = [0u8; 44]; 

    

    for i in &path_vec{
        // println!("{:?}", i.split(".mp3").collect::<Vec<_>>());//<Vec<_>>());
        // println!("{:?}", i);

        match &i.to_lowercase().as_str() {
            x if x.contains(".mp3") => println!("wav!"),
            x if x.contains(".aiff") => println!("Aiff!"),
            x if x.contains(".flac")  => println!("flac!"),
            _ => ()
        }
    }

    Some(audio_vecs)

    // for i in path_vec{
    //     println!("{}", i);
    //     let file = File::open(i)
    //                     .unwrap();  // TODO: Use unwrap or else
    //     let mut reader = BufReader::new(file);
    //     reader.read_exact(&mut header).unwrap();
    //     reader.read_to_end(&mut audio_vecs).unwrap();
    // }

    // return Some(audio_vecs)
}