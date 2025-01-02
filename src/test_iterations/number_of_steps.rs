/*
    https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
    1. 입력 받은 숫자에 대해서 홀수/짝수 체크
    2. 홀인 경우 입력 받은 값에 대해 -1 실행
    3. 짝인 경우 나누기 실행
    4. 각 실행에 대해서 count를 증가 시켜줌
*/

pub fn test() {
    println!("number_of_steps result is {}", number_of_steps(14));
    println!("number_of_steps result is {}", number_of_steps(8));
    println!("number_of_steps result is {}", number_of_steps(123));
    println!("number_of_steps result is {}", number_of_steps(128));
}
fn number_of_steps(num: u32) -> u32 {
    /*
       leading_zeros : 2진법 최대 자리 1 이전까지 0으로 나오는 수
       count_ones : 2진법 1의 갯수
       (32 - leading_zeros) + count_ones - 1: 우리가 원하는 수를 계산하는 수식
    */
    32 - num.leading_zeros() + num.count_ones() - 1
}

#[test]
fn tc() {
    let result = number_of_steps(14);
    let check = 6;

    assert_eq!(result, check);

    let result = number_of_steps(8);
    let check = 4;

    assert_eq!(result, check);

    let result = number_of_steps(123);
    let check = 12;

    assert_eq!(result, check);

    let result = number_of_steps(128);
    let check = 8;

    assert_eq!(result, check);
}
