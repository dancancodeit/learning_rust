use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("failed creating file {err}"),
            },
            other_error => panic!("not sure wtf is going on here but i can't handle it: {other_error:?}"),
        },
    };
}