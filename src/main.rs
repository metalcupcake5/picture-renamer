use exif::{DateTime, Tag, Value};
use std::fs;

fn main() {
    fs::create_dir("./output");
    let contents = fs::read_dir("./pictures").unwrap();
    for entry in contents {
        let path = entry.unwrap().path();
        let path_copy = path.clone();

        let file = fs::File::open(path).unwrap();
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        if let Some(field) = exif.get_field(Tag::DateTimeOriginal, exif::In::PRIMARY) {
            if let Value::Ascii(ref vec) = field.value {
                if let Some(date_str) = vec.first() {
                    if let Ok(date_time) = DateTime::from_ascii(date_str) {
                        if let Some(extension) = path_copy.extension() {
                            if let Some(ext_str) = extension.to_str() {
                                let filename = format!(
                                    "{}-{}-{}_{}{}{}.{}",
                                    date_time.year,
                                    date_time.month,
                                    date_time.day,
                                    date_time.hour,
                                    date_time.minute,
                                    date_time.second,
                                    ext_str
                                );
                                fs::copy(path_copy.as_path(), format!("./output/{}", filename));
                                println!("{}", filename);
                            } else {
                                println!("Failed to convert extension to string.");
                            }
                        } else {
                            println!("No file extension found.");
                        }
                    } else {
                        println!("Failed to parse date.");
                    }
                }
            } else {
                println!("DateTimeOriginal tag is not in expected format.");
            }
        } else {
            println!("DateTimeOriginal tag not found.");
        }
    }
}
