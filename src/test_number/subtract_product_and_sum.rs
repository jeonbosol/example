/*
   https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
   1. 숫자를 배열로 변환 작업
   2. 배열 곱셈 구하기
   3. 배열 합 구하기
*/
pub fn test() {
    println!(
        "subtract_product_and_sum result : {:?}",
        subtract_product_and_sum(234)
    );
    println!(
        "subtract_product_and_sum result : {:?}",
        subtract_product_and_sum(4421)
    );

    match subtract_product_and_sum(-234) {
        None => {
            println!("Invalid parameter: The `n` values provided should be positive integers.");
        }
        Some(value) => {
            println!("subtract_product_and_sum result : {value}");
        }
    }
}

fn subtract_product_and_sum(n: i32) -> Option<u32> {
    get_array_by_num(n).map(|array| {
        let product: u32 = array.iter().product();
        let sum: u32 = array.iter().sum();
        product - sum
    })
}

/*
   i32로 타입을 변경한 이유는 `None` 이 발생할 수 있는 테스트를 위함.
   * u32에서는 filter_map 사용해도 무방함. : u32로 들어온 경우 c.to_digit(10) 실행시 None 이 되는 경우는 없음.
     - radix 값이 너무 크게 주어진다면 None 이 발생할 수 있지만, 10으로 고정한 상태임.
*/
fn get_array_by_num(num: i32) -> Option<Vec<u32>> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
}

#[test]
fn tc() {
    let result = subtract_product_and_sum(234);
    let check = 15;
    assert_eq!(result, Some(check));

    let result = subtract_product_and_sum(-234);
    assert_eq!(result, None);
}
