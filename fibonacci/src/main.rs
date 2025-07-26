fn main() {
    loop {
        println!("Enter Fibonacci N");
        let mut fib_n: String = String::new();
        std::io::stdin()
            .read_line(&mut fib_n)
            .expect("Failed to read line");
        let fib_n: i32 = fib_n.trim().parse().expect("Wrong data");
        match fib_n {
            0 => {
                print!("0");
            }
            1 | 2 => {
                print!("1");
            }
            _ => {
                let mut prev0: i32 = 1;
                let mut prev1: i32 = 1;
                let mut result: i32 = 0;

                for number in 2..fib_n {
                    //println!("{}+{}", prev0, prev1);
                    result = prev0 + prev1;
                    prev0 = prev1;
                    prev1 = result;
                }
                println!("{}", result);
            }
        };
    }
}
