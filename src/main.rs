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

#[test]
fn it_works() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(2), "2");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(4), "4");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(6), "Fizz");
    assert_eq!(fizzbuzz(7), "7");
    assert_eq!(fizzbuzz(8), "8");
    assert_eq!(fizzbuzz(9), "Fizz");
    assert_eq!(fizzbuzz(10), "Buzz");
    assert_eq!(fizzbuzz(11), "11");
    assert_eq!(fizzbuzz(12), "Fizz");
    assert_eq!(fizzbuzz(13), "13");
    assert_eq!(fizzbuzz(14), "14");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
}
