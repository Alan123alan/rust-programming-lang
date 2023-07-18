fn main() {
    let x = -2;
    //unlike other dynamic languages I have used rust requires a bool value
    //seem like there is no truthy or falsey values
    if x > 0 {
        println!("Condition was true.");
    }else{
        println!("Condition was false.");
    }
    //if is and expression in rust so it can evaluate to a value
    let value_from_if_expr = if x>0 {10} else {22};
    println!("This is the value from an if expression: {value_from_if_expr}");
}
