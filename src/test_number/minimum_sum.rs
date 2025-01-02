/*
   https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
   2자리 숫자 최소 값을 더하기
   1. 숫자 문자열 치환
   2. 오름차순 정렬
   3. char_array[0][2] + char_array[1][3] 더하기
*/

pub fn test() {
    // println!("minimum_sum result : {:?}", minimum_sum(2932));
    // println!("minimum_sum result : {:?}", minimum_sum(4009));
    match minimum_sum(123) {
        None => {
            println!(
                "Invalid parameter: Input `num` value should be within a 4-digit number range."
            );
        }
        Some(result) => {
            println!("minimum_sum result : {result}.");
        }
    }
}

fn minimum_sum(num: i32) -> Option<i32> {
    //주어진 조건은 4자리 숫자이기 때문에 해당 조건이 맞지 않으면 None 반환
    if !(1000..=9999).contains(&num) {
        return None;
    }

    let char_array = get_ordered_char_array_by_nums(num);
    let num1: i32 = format!(
        "{}{}",
        char_array.first().expect("The first item should exist."),
        char_array.get(2).expect("The third item should exist.")
    )
    .parse()
    .ok()?;
    let num2: i32 = format!(
        "{}{}",
        char_array.get(1).expect("The index value should be `1`."),
        char_array.get(3).expect("The index value should be `3`.")
    )
    .parse()
    .ok()?;

    Some(num1 + num2)
}

fn get_ordered_char_array_by_nums(num: i32) -> Vec<char> {
    let mut char_array: Vec<char> = num.to_string().chars().collect();
    char_array.sort_unstable();

    char_array
}

#[test]
fn tc() {
    let result = minimum_sum(2932);
    let check = 52;
    assert_eq!(result, Some(check));

    let result = minimum_sum(123);
    assert_eq!(result, None);
}
