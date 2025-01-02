/*
   https://leetcode.com/problems/add-two-integers/
*/
fn add_two_integers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

pub fn test() {
    println!("add_two_integers result : {}", add_two_integers(12, 5));
    println!("add_two_integers result : {}", add_two_integers(-10, 4));
}

#[test]
fn tc() {
    let result = add_two_integers(12, 5);
    let check = 17;
    assert_eq!(result, check);
}
