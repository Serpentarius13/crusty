mod garden;

use std::fs::{self, File};
use std::io::{self, ErrorKind};
fn main() {
    let file = File::open("hello.txt");

    let greeting_file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e.to_string()),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error.to_string())
            }
        },
    };

    println!("{:?}", greeting_file);

    let string = read_file();
    println!("{:?}", string);

    struct Point<T, U> {
        x: T,
        y: U,
    }

    let checker = Point { x: 1, y: 2.0 };

    trait ConsoleLog {
        fn logger(&self) -> String;
    }

    struct Person {
        name: String,
        age: i32,
        hobby: String,
    }

    impl ConsoleLog for Person {
        fn logger(&self) -> String {
            let hobby = self.hobby.replace('\n', "");
            let return_str = "My hobby is ".to_string() + &hobby;
            return_str
        }
    }

    let mut guy = Person {
        name: String::from("Naruto"),
        age: 20,
        hobby: String::from("Ichiraku Ramen"),
    };

    println!("{}", guy.logger())
}

fn read_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
