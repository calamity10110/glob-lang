cs rust:
    fn fast_sum(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }
    
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

mn main():
    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    total = rust.fast_sum(numbers)
    print("Sum:", total)
    
    fib_10 = rust.fibonacci(10)
    print("Fibonacci(10):", fib_10)
