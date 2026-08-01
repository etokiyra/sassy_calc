use std::io;
fn main() {
    println!("Do a calculation!");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        println!("You typed: {}", input.trim());
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 3 {
            println!("Thats an invalid input. Expected: <num> <op> <num>");
            continue;
        }

        let a: f64 = parts[0].parse().unwrap();
        let b: f64 = parts[2].parse().unwrap();
        let op = parts[1];

        println!("a={} op={} b={}", a, op, b);

        let result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => {
                println!("Unknown operator: {}", op);
                continue;
            }
        };

        println!("= {}", result);
    }
}
