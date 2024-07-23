/*test the src/lib.rs functions*/

use trust::add;
use trust::divide;
use trust::multiply;
use trust::subtract;

#[test]
fn test_add() {
    assert_eq!(add(10, 5), 15);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(10, 5), 5);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(10, 5), 50);
}

#[test]
fn test_divide() {
    assert_eq!(divide(10, 5), 2);
}
