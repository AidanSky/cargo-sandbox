pub fn fibonacci(number: u64) -> u64 {
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