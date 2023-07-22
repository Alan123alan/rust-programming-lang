//example call 3rd fibonacci number
// fibonacci(3) ----> fibonacci(2) + fibonacci(1) ----> 1 + 1 ----> 2
  // fibonacci(2) ----> fibonacci(1) + fibonacci(0) ----> 1 + 0 ----> 1
    // fibonacci(1) ----> 1
    // fibonacci(0) ----> 0
  // fibonacci(1) ----> 1
fn fibonacci(n:i32) -> i32{
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn main() {
    for n in 0..=10{
       let nth_fibonacci = fibonacci(n); 
       println!("This is the Fibonacci in the {n} position: {nth_fibonacci}");
    }
}
