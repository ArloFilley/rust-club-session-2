////////////////////////
// 1. Largest Element //
////////////////////////

fn largest_item(arr: Vec<i64>) -> i64 {
    //! Returns the largest item in a list

    unimplemented!();
}

fn main() {
    let arr = vec![1, 45, 3, 78, 3, 45, 98, 12];
    let result = largest_item(arr);

    println!("The largest element is: {}", result);

    assert_eq!(result, 98, "The largest item was not correct");
}
