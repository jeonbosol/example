/*
   https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
   문자열 split whitespace를 이용 및 카운팅하여 가장 큰 값을 찾기
*/

pub fn test() {
    let sentences: Vec<String> = vec![
        String::from("alice and bob love leetcode"),
        String::from("i think so too"),
        String::from("this is great thanks very much"),
    ];

    println!("most_words_found result : {}", most_words_found(&sentences));
}

fn most_words_found(sentences: &[String]) -> usize {
    sentences
        .iter()
        .map(|s| s.split_whitespace().count())
        .max()
        .unwrap_or_default()
}

#[test]
fn tc() {
    let sentences: Vec<String> = vec![
        String::from("alice and bob love leetcode"),
        String::from("i think so too"),
        String::from("this is great thanks very much"),
    ];

    let result = most_words_found(&sentences);
    let check = 6;
    assert_eq!(result, check);

    let sentences: Vec<String> = vec![];
    let result = most_words_found(&sentences);
    assert_eq!(result, 0);
}
