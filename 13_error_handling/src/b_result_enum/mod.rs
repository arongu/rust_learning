use std::fs::File;
use std::io::ErrorKind;

const FILE_PATH: &str = "/home/aron/hello.txt";

/*
enum Result { represents success or failure
    Ok(T)
    Err(E)
}
*/

pub fn open_file_or_panic() {
    let f = File::open(FILE_PATH);

    match f {
        Ok(_) => println!("Managed to open {}", FILE_PATH),
        _ => {
            panic!("Could not open file {}", FILE_PATH);
        }
    }
}

pub fn open_file() {
    let f = File::open(FILE_PATH);
    let f= match f {
        Ok(file) => file,
        Err(error) =>  match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_PATH) {
                Ok(new_file) => new_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            all_other_errors => panic!("Problem creating the file: {:?}", all_other_errors),
        }
    };
}
