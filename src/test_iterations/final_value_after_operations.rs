/*
    https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
    1. 주어진 X는 초기값 0
    2. 배열에서 X문자 제거 및 -- ++를 카운팅
    3. 카운팅 된 값을 sum

*/
use std::collections::HashMap;

pub fn test() {
    let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];

    println!(
        "final_value_after_operations result : {}",
        final_value_after_operations(&operations)
    );
}
fn final_value_after_operations(operations: &[String]) -> i32 {
    let map = operations
        .iter()
        .fold(HashMap::with_capacity(2), |mut m, operation| {
            let key = operation.replace('X', "");
            *m.entry(key).or_insert(0) += 1;
            m
        });

    let plus_count = map.get("++").copied().unwrap_or(0);
    let minus_count = map.get("--").copied().unwrap_or(0);

    plus_count - minus_count
}

#[test]
fn tc() {
    let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
    let result = final_value_after_operations(&operations);
    let check = 1;

    assert_eq!(result, check);
}
