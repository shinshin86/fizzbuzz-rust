fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        return format!("FizzBuzz");
    } else if n % 3 == 0 {
        return format!("Fizz");
    } else if n % 5 == 0 {
        return format!("Buzz");
    } else {
        return n.to_string();
    }
}

fn main() {
    for n in 1..101 {
        println!("{}", fizzbuzz(n));
    }
}
