use std::collections::HashMap;
use std::io;

fn mean(list: &Vec<i32>) -> i32 {
    let mut mean = 0;
    for i in list.iter() {
        mean += i;
    }
    mean / list.len() as i32
}

fn median(list: &Vec<i32>) -> i32 {
    let mut list_clone = list.clone();
    list_clone.sort_unstable();
    list_clone[list_clone.len() / 2]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut most_often: (i32, i32) = (0, 0);

    for i in list.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    for (k, val) in &map {
        if val >= &most_often.1 {
            most_often = (**k, *val);
        }
    }

    most_often.0
}

fn pig_latin(word: &String) -> String {
    let first_char = match word.chars().next() {
        Some(v) => v,
        None => 'n',
    };

    return match first_char {
        'a' => format!("{}{}", word, "hay"),
        'e' => format!("{}{}", word, "hay"),
        'i' => format!("{}{}", word, "hay"),
        'o' => format!("{}{}", word, "hay"),
        'u' => format!("{}{}", word, "hay"),
        _ => {
            let mut without_first_char = String::from(&word[1..]);
            without_first_char.push(first_char);
            format!("{}{}", without_first_char, "ay")
        }
    };
}

fn get_employee_and_department(string: String) -> (String, String) {
    let splitted_string: Vec<&str> = string.split(' ').collect();
    let name = splitted_string[1];
    let department = splitted_string[3];
    (String::from(name), String::from(department))
}

fn main() {
    let numbers = vec![1, 3, 5, 3, 2];
    let mean = mean(&numbers);
    let median = median(&numbers);
    let mode = mode(&numbers);
    println!("Mean is: {}", mean);
    println!("Median is: {}", median);
    println!("Mode is: {}", mode);
    println!("Pig latin is: {}", pig_latin(&String::from("apple")));

    let mut company_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Add *Employee* to *Department*...");
        let mut user_string = String::new();
        io::stdin()
            .read_line(&mut user_string)
            .expect("Failed to read line");

        if user_string.trim() == "cancel" {
            break;
        } else if user_string.trim() == "show" {
            println!("{:#?}", company_map);
        } else {
            let employee_department = get_employee_and_department(String::from(user_string.trim()));

            company_map
                .entry(employee_department.1.clone())
                .or_insert(Vec::new())
                .push(employee_department.0);
        }
    }
}
