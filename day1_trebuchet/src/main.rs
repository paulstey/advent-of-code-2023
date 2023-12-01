struct Calibration {
    first: i32,
    last: i32,
}

impl Calibration {
    fn from(input: &str) -> Self {
        let numbers = extract_numbers(input);
        let first = numbers.chars()[0].parse::<i32>();
        let last = numbers.pop().parse::<i32>();

        Self { first, last }
    }
}

fn extract_numbers(input: &str) -> String {
    let mut numbers = String::from("");

    input.chars().for_each(|c| {
        if c.is_ascii_digit() {
            numbers.push(c)
        }
    });

    numbers
}

fn main() {
    let input = "123 abc 456 def 789";
    let numbers = extract_numbers(input);

    let calib = Calibration::from(input);

    println!("{:?}", calib);
}
