use std::env;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    // should only be one arg, parse into int
    if args.len() != 3 {
        println!("Enter an integer and amount of sims");
        return;
    }

    // parge people into int
    let amount: i32 = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("error, not a valid int");
            return;
        }
    };

    // parse amount of simulations into int
    let sims: i32 = match args[2].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("error, not a valid int");
            return;
        }
    };

    // create counter for successful sims
    let mut counter = 0;
    for _ in 0..sims {
        let success = test(amount);
        if success == true {
            counter += 1;
        }
    }
    let count_percent= (counter as f64) / (sims as f64);
    let true_percent = count_percent * (100 as f64);
    println!("The percent of successful simulations was {:.2}%", true_percent);
}

fn test(amount: i32) -> bool {
    // take arguments for amount of people
    // set a birthday (1-365) for each person
    // check if any two people have the same birthday

    // define argument for how many people should be included in the test


    // create a person for size people, and assign a random int 1-365
    let mut rng = rand::rng();
    let mut birthdays: Vec<i32> = Vec::new(); // store in vec or in hashmap?
    for _n in 0..amount {
        let date = rng.random_range(1..=365);
        birthdays.push(date);
        // println!("Birthday {n}: {}", date);
    }

    // create a hashmap for all people, and assign a birthday from vec
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for &birthday in &birthdays {
        *counter.entry(birthday).or_insert(0) += 1;
    }

    // assign each integer a hashmap with a date
        // easier to set at time of rand entry?

    // check if any two hashmap occurences have the same value, and if so, print them
    // create duplicate hashmap that stores people.keys and a counter, if counter is positive, add to vector for printing
    let mut duplicates = false;
    for (&_birthday, &count) in &counter {
        if count > 1 {
            // println!("Duplicate birthday found at {} with {} people!", birthday, count);
            duplicates = true;
        }
    }
    if duplicates == false {
        // println!("no duplicates found D:");
    }
    return duplicates;
}