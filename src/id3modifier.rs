use std::collections::btree_map::Iter;

use id3::{Tag, TagLike, Error, ErrorKind, Version, Frame};
use id3::frame::{Content, Chapter};
use std::fs::copy;
use std::str::FromStr;

pub fn load_tags(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tag = Tag::read_from_path(path)?;

    let chapter = tag.chapters(); {
        for x in chapter {
            println!("{x}");
        }
    }

    Ok(())
}

pub fn modify_tags(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let temp_path = "/tmp/music.mp3";

    copy(path, temp_path)?;

    let mut tag = match Tag::read_from_path(temp_path) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err)),
    };
    
    tag.add_frame(new_chapter(chapter_count, start_time, end_time, chapter_name));

    Ok(())
}

pub fn new_chapter(chapter_count: i32, start_time: &str, end_time: &str, chapter_name: &str) -> Chapter {
    let start_count = start_time.matches(':').count();
    let end_count = end_time.matches(':').count();
    
    let start_sec = match start_count {
        0 => u32::from_str(start_time).unwrap(),
        1 => {
            let split_vec = start_time.split(':').collect::<Vec<_>>();
            let min: u32 = u32::from_str(split_vec[0]).unwrap();
            min * 60 + u32::from_str(split_vec[1]).unwrap()
        },
        2 => {
            let split_vec = start_time.split(':').collect::<Vec<_>>();
            let hour = u32::from_str(split_vec[0]).unwrap();
            let min = u32::from_str(split_vec[1]).unwrap();
            hour * 60 * 60 + min * 60 + u32::from_str(split_vec[2]).unwrap()
        },
        _ => panic!()
    };

    let end_sec = match end_count {
        0 => u32::from_str(end_time).unwrap(),
        1 => {
            let split_vec = end_time.split(':').collect::<Vec<_>>();
            let min: u32 = u32::from_str(split_vec[0]).unwrap();
            min * 60 + u32::from_str(split_vec[1]).unwrap()
        },
        2 => {
            let split_vec = end_time.split(':').collect::<Vec<_>>();
            let hour = u32::from_str(split_vec[0]).unwrap();
            let min = u32::from_str(split_vec[1]).unwrap();
            hour * 60 * 60 + min * 60 + u32::from_str(split_vec[2]).unwrap()
        },
        _ => panic!()
    };

    Chapter { 
        element_id: chapter_count.to_string(), 
        start_time: start_sec, 
        end_time: end_sec, 
        start_offset: 0xff, 
        end_offset: 0xff, 
        frames: Vec::new() 
    }
}