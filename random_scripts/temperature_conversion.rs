use std::env;

fn main() {
    // command line arg for temp + C or F 
    let args: Vec<String> = env::args().collect();

    // return error if incorrect arguments provided
    if args.len() != 3 {
        println!("Enter a float and 'C' or 'F' to clarify system");
        return;
    }

    // parse first arg into float, return error if invalid
    let temperature: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("error, not a valid float");
            return;
        }
    };
    // parse second arg into char
    let system: char = match args[2].parse() {
        Ok(c) => c,
        Err(__) => {
            println!("error, invalid system");
            return;
        }
    };

    // check second arg to determine what function to use
    let converted: f64 = match system {
        'f' | 'F' => to_celcius(temperature),
        'c' | 'C' => to_farenheit(temperature),
        _ => {
            println!("Must use either 'c' or 'f'.");
            return;
        }
    };
    // print return value 
    println!("The converted temperature is {:.2}", converted);
}

// define function for conversion to C 
fn to_celcius(temperature: f64) -> f64 {
    (5.0/9.0)*(temperature-32.0)
}

// define function for conversion to F
fn to_farenheit(temperature: f64) -> f64 {
    (temperature*(9.0/5.0))+32.0
}