
fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    statement_expression();

    let x = function_with_return_value_are_expressions();
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}


fn statement_expression() {

    // Statement: a piece of code that performs some action
    let x = 5;

    // Expression: a piece of code that produces a value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}


fn function_with_return_value_are_expressions() -> i32 {
     5 // note: return values are expressions with no semicolons
}
