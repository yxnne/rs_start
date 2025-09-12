use std::fs::File;
use std::io::ErrorKind;
pub fn try_read_test_txt() {
    let f = File::open("hello.txt");
    let file = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

pub fn simple_unwrap_read_test_txt() {
    let f = File::open("test.txt").unwrap();
}

pub fn simple_expect_read_test_txt() {
    let f = File::open("test.txt").expect("Failed to open test.txt");
}