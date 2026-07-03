use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Input ERROR");

    let mut nums = input.split_whitespace().map(|s: &str| s.parse::<i32>().unwrap());
    let x = nums.next().unwrap();
    let a = nums.next().unwrap();

    let area = x*a;
    if area / 3 * 3 == area {
        println!("{}", area / 3)
    }
    else {
        println!("{}", area / 3 +1)
    }
}
