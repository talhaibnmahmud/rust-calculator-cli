#[allow(dead_code)]
fn operate(operator: char, a: f64, b: f64) -> f64 {
    match operator {
        '/' => a / b,
        '*' | 'x' | 'X' => a * b,
        '+' => a + b,
        '-' => a - b,
        _ => panic!("Invalid operator!"),
    }
}

#[allow(dead_code)]
fn outout(a: f64, o: char, b: f64, r: f64) -> String {
    format!("{} {} {} = {}", a, o, b, r)
}

#[allow(dead_code)]
pub fn parse_args() {
    let mut args = std::env::args();

    let first = args.nth(1);
    match first {
        Some(ref inner) => println!("Got first argument: {}", inner),
        None => panic!("Didn't get the first arguments"),
    }

    let operator = args.next();
    match operator {
        Some(ref inner) => println!("Got operator: {}", inner),
        None => panic!("Didn't get the operator"),
    }

    let second = args.next();
    match second {
        Some(ref inner) => println!("Got second argument: {}", inner),
        None => panic!("Didn't get the second arguments"),
    }

    println!(
        "Arguments passed are: {:?}, {:?}, {:?}",
        first, operator, second
    );

    let first_number = first.unwrap().parse::<f64>().unwrap();
    let second_number = second.unwrap().parse::<f64>().unwrap();
    let operator = operator.unwrap().chars().next().unwrap();
    println!("Parsed numbers: {:?}, {:?}", first_number, second_number);

    let result = operate(operator, first_number, second_number);
    println!("{}", outout(first_number, operator, second_number, result));
}
