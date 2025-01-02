/*
   https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
   배열로 변경하여 적용
*/

pub fn test() {
    let arr = [1, 0, 1];
    println!("get_decimal_value result : {}", get_decimal_value(&arr));
}

fn get_decimal_value(arr: &[u32]) -> u32 {
    /*
       let mut num = 0;
       arr.iter().for_each(|x| num = num * 2 + x);
       num
    */
    arr.iter().fold(0, |num, &x| num * 2 + x)
}

#[test]
fn tc() {
    let arr = [1, 0, 1];
    let result = get_decimal_value(&arr);
    let check = 5;
    assert_eq!(result, check);

    let arr = [1, 0, 0, 1];
    let result = get_decimal_value(&arr);
    let check = 9;
    assert_eq!(result, check);
}
