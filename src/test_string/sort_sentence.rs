/*
   https://leetcode.com/problems/sorting-the-sentence/
   1. split 문자열 크기만큼 vec 생성
   2. split 문자열에서 index 번호 파싱
   3. index 번호에 맞는 위치에 문자열 저장
   4. vec join 으로 결과 추출
*/
use std::collections::HashMap;

pub fn test() {
    let s = String::from("is2 sentence4 This1 a3");
    println!("sort_sentence result : {:?}", sort_sentence(&s));
}

fn sort_sentence(s: &str) -> Result<String, &str> {
    let mut map: HashMap<i64, String> = HashMap::new();

    for x in s.split_whitespace() {
        let (key, value): (String, String) = x.chars().partition(|&c| c.is_numeric());
        let key: i64 = key.parse().map_err(|_| "Key should contain a number.")?;
        map.entry(key).or_insert(value);
    }

    let mut sorted: Vec<_> = map.into_iter().collect();
    sorted.sort_unstable_by_key(|&(k, _)| k);

    Ok(sorted
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .join(" "))
}

#[test]
fn tc() {
    let s = String::from("is2 sentence4 This1 a3");

    let result = sort_sentence(&s);
    let check = "This is a sentence".to_string();
    assert_eq!(result, Ok(check));

    let s = String::from("is sentence4 This1 a3");

    let result = sort_sentence(&s);
    assert_eq!(result.is_err(), true);
}
