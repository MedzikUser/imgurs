mod parse;

pub mod album_info;
pub mod clipboard;
pub mod credits;
pub mod delete_image;
pub mod info_image;
pub mod upload_image;

pub use parse::*;

use chrono::{prelude::DateTime, Utc};
use colored::Colorize;
use imgurs::ImageInfoData;
use std::time::{Duration, UNIX_EPOCH};

// print image information from imgur
pub fn print_image_info(i: &ImageInfoData) {
    // format image upload date
    let d = UNIX_EPOCH + Duration::from_secs(i.datetime.try_into().unwrap());
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    // image title
    if i.title != None {
        let title = i.title.clone();

        println!(
            "{} {}",
            "title".green(),
            title.unwrap_or_else(|| "unknown".to_string()).magenta()
        );
    }

    // image description
    if i.description != None {
        let desc = i.description.clone();

        println!(
            "{} {}",
            "description".green(),
            desc.unwrap_or_else(|| "unknown".to_string()).magenta()
        );
    }

    // image deletehas
    if i.deletehash != None {
        let delhash = i.deletehash.clone();

        println!(
            "{} {}",
            "deletehash".green(),
            delhash.unwrap_or_else(|| "unknown".to_string()).magenta()
        );
    }

    println!("{} {}", "id".green(), i.id.magenta());
    println!(
        "{} {} {}",
        "upload date".green(),
        timestamp_str.magenta(),
        "(UTC)".blue()
    );
    println!("{} {}", "type".green(), i.img_type.magenta());
    println!("{} {}", "width".green(), i.width.to_string().magenta());
    println!("{} {}", "height".green(), i.height.to_string().magenta());
    println!(
        "{} {} {}",
        "size".green(),
        (i.size / 1000).to_string().magenta(),
        "KB".blue()
    );
    println!("{} {}", "views".green(), i.views.to_string().magenta());
    println!(
        "{} {}",
        "bandwidth".green(),
        i.bandwidth.to_string().magenta()
    );
    println!("{} {}", "link".green(), i.link.magenta());
}
