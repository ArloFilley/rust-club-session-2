/////////////////////
// 4. Looping Sums //
/////////////////////

fn sum_for(arr: Vec<i64>) -> i64 {
    //! Calculates the sum of elements in a list. Using `for`

    unimplemented!();
}

fn sum_while(arr: Vec<i64>) -> i64 {
    //! Calculates the sum of elements in a list. Using `while`

    unimplemented!();
}

fn sum_loop(arr: Vec<i64>) -> i64 {
    //! Calculates the sum of elements in a list. Using `loop`

    // HINT: `loop { }`, `loop` is an infinite loop. You need to explicitly
    // break out of the loop

    unimplemented!();
}

fn main() {
    let arr = vec![1, 45, 3, 78, 3, 45, 98, 12];
    let result = sum_for(arr.clone());

    println!("The list sum is: {:?}", result);

    assert_eq!(result, 285, "The list sum was not correct");

    let result = sum_while(arr.clone());

    println!("The list sum is: {:?}", result);

    assert_eq!(result, 285, "The list sum was not correct");

    let result = sum_loop(arr.clone());

    println!("The list sum is: {:?}", result);

    assert_eq!(result, 285, "The list sum was not correct");
}
