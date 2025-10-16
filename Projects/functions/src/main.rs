fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn function_that_returns() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");
    another_function(5);
    let x: i32 = function_that_returns();
    println!("The value from the returns function is {x}");
}
