use std::env;

fn main() {
    // command line arg for temp + C or F 
    let args: Vec<String> = env::args().collect();

    // return error if incorrect arguments provided
    if args.len() != 2 {
        println!("Enter an integer");
        return;
    }

    // parse first arg into float, return error if invalid
    let number: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("error, not a valid int");
            return;
        }
    };
    let equated: i32 = fibonacci(number);
    println!("The {}th fibonacci is {}", number, equated);
}

fn fibonacci(number: i32) -> i32 {
    if number <= 0 {
        return 0;
    }
    if number == 1 {
        return 1;
    }

    let mut x = 0;
    let mut y = 1;

    for _ in 1..number {
        let temp = x + y;
        x = y;
        y = temp;
    }

    y
}
