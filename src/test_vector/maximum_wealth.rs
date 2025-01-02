/*
   https://leetcode.com/problems/richest-customer-wealth/description/
*/
pub fn test() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];

    match maximum_wealth(&accounts) {
        Some(wealth) => println!("Maximum wealth: {wealth}"),
        None => println!("The value of accounts is empty. Cannot get max value."),
    }
}

fn maximum_wealth(accounts: &[Vec<i32>]) -> Option<i32> {
    accounts.iter().map(|account| account.iter().sum()).max()
}

#[test]
fn tc() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let empty_accounts: Vec<Vec<i32>> = Vec::new();
    let check = 6;

    assert_eq!(maximum_wealth(&accounts), Some(check));
    assert_eq!(maximum_wealth(&empty_accounts), None);
}
