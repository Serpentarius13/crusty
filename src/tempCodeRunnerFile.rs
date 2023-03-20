mod garden;

use std::collections::HashMap;
use std::io;

fn main() {
    let mut hashbrown = HashMap::new();

    loop {
        let mut name = String::new();

        let mut field = String::new();

        println!("Hey, who are we adding?");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        println!("And where?");

        io::stdin()
            .read_line(&mut field)
            .expect("Faled to read line");
        println!("{name} was added to {field}");

        hashbrown.insert(name, field);

        println!("new state of things is: {:?}", hashbrown);
        println!("do you want to continue? y/n");

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        if (response.chars().next().unwrap() == 'y') {
            continue;
        } else {
            break;
        }
    }

    // println!("Hello, world!");

    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // print!("{:?}", arr);

    // garden::arrConvertPrint(&arr);

    // let v: Vec<i32> = Vec::new();

    // let mut c = vec![1, 2, 3];

    // c.push(1);
    // c.push(3);

    // let third: &i32 = &c[1];

    // let third: Option<&i32> = c.get(1);

    // match third {
    //     Some(third) => println!("The third element is {third}!"),
    //     None => println!("No third for you!"),
    // }

    // let mut d = vec![1, 2, 3, 4, 5, 6, 7];

    // for i in &mut d {
    //     *i += 50;
    // }

    // println!("{:?}", d);

    // let mut str = String::from("fooooo");
    // str.push_str("BAR!");
    // println!("{str}");

    // let one_string = String::from("Aboba");

    // let mut new_string = one_string + " Bobin";

    // println!("{new_string}");

    // new_string = new_string.replace("Aboba", "Boba");

    // println!("{new_string}");

    // let mut hashbrown = HashMap::new();

    // hashbrown.insert(String::from("boba"), 1);
    // hashbrown.insert(String::from("bobaaa"), 2);

    // for (key, value) in &hashbrown {
    //     println!("{key}, {value}")
    // }

    // let keyie = String::from("Abobus");
    // let valuie = 123;

    // hashbrown.insert(keyie, valuie);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    // let scored = scores.entry(String::from("Yellow"));
    // println!("{:?}", scored);

    // let problem_str = String::from("apple goes bananas");
}