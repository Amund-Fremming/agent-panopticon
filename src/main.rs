pub mod calculator;

fn main() {
    println!("Hello, members of the panopticon!");
    println!("Calculator Demo:");
    println!("5 + 3 = {}", calculator::add(5, 3));
    println!("5 - 3 = {}", calculator::subtract(5, 3));
}
