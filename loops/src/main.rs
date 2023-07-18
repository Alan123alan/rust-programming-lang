fn main() {
    let mut counter = 0;
    let upper_limit = 5;
    //loops are expressions too baby!
    //just add the value to be returned after the break statement
    let result = loop{
        if counter > upper_limit {
            break counter;
        }
        println!("Around the world,");
        counter += 1;
    };
    println!("This program printed 'Around the world,' {result} times.");

    //You can specify loops with a label and then break loops specifying
    //from which loop do you want to break
    //loop labels must start with an single quote '
    let mut outer_counter = 0;
    let mut inner_counter = 0;
    'outer_loop: loop{
        'inner_loop: loop{
            if inner_counter > upper_limit{
                break 'outer_loop;
            } 
            inner_counter += 1;
        }
        outer_counter += 1;
    }
    println!("Outer counter should be 0 and inner counter should be 6");
    println!("Outer counter: {outer_counter}");
    println!("Inner counter: {inner_counter}");
    //for loops remind me of Python
    let arr = [1,2,4,8,16];
    for element in arr {
        println!("{element}");
    }
    for num in (0..=5).rev(){
        println!("{num}");
    }
}
