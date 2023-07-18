fn main() {
    println!("Hello, world!");
    //Rust takes "" double quotes as a &str, and '' and a char
    another_function(5,'M',"something about what this unit meassures");
    //Scopes are expressions and can retur a value by ommiting ';' at the last
    //expression
    let scope_val = {
	let x = 1;
        x - 2
    };
    println!("This is a value from a scope expression: {scope_val}");
    println!("Double of value 4 is: {}",double_value(4));
}

fn another_function(magnitude: i32, unit: char, description: &str){
    println!("you passed this value: {magnitude}{unit} {description}");
}

fn double_value(value: i32)->i32{
    return value*2;
}
