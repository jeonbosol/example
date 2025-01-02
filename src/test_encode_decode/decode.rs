/*
   https://leetcode.com/problems/decode-xored-array/
   1. decoded 배열 생성
   2. encoded 전달 받은 배열과 xor 연산하여 decoded update
*/

pub fn test() {
    let encoded = vec![1, 2, 3];
    let first = 1;

    println!("decode result : {:?}", decode(&encoded, first));
}

fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![first];
    result.extend(encoded.iter().scan(first, |state, &x| {
        *state ^= x;
        Some(*state)
    }));
    result
}

#[test]
fn tc() {
    let encoded = vec![1, 2, 3];
    let first = 1;
    let result = decode(&encoded, first);
    let check = vec![1, 0, 2, 1];
    assert_eq!(result, check);
}
