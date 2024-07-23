// write test functions of main.rs

use trust::add;
use trust::divide;
use trust::multiply;
use trust::subtract;

fn main() {
    println!("Addition: {}", add(10, 5));
    println!("Subtraction: {}", subtract(10, 5));
    println!("Multiplication: {}", multiply(10, 5));
    println!("Division: {}", divide(10, 5));
}
