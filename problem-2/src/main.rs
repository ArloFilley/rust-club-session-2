///////////////////////
// 2. Reverse a List //
///////////////////////

fn reverse(arr: Vec<i64>) -> Vec<i64> {
    //! Reverses a list

    let mut new_arr: Vec<i64> = vec![];

    // HINT: `new_arr.push(*arr[(arr.len() as i64 - i) as usize]);`

    unimplemented!();
}

fn main() {
    let arr = vec![1, 45, 3, 78, 3, 45, 98, 12];
    let result = reverse(arr);

    println!("The reversed list is: {:?}", result);

    assert_eq!(
        result,
        [12, 98, 45, 3, 78, 3, 45, 1],
        "The reversed list was not correct"
    );
}
