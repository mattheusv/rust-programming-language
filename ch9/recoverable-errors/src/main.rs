use std::fs::File;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    //let f = File::open("file")?; // Not compile, should be inside a function that returns a Result<>
}

fn read_username_from_file_very_very_short() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

fn read_username_from_file_very_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("username.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

fn get_or_create_file_expected() -> File {
    File::open("file").expect("Expect 'file' file")
}

fn get_or_create_file_unwrap() -> File {
    File::open("file").unwrap()
}

fn get_or_create_file_match() -> File {
    let f = File::open("file");

    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("file") {
                Ok(fc) => fc,
                Err(errr) => panic!("Got and error: {:?}", errr),
            },
            other_error => panic!("Other error: {:?}", other_error),
        },
    };
    f
}
