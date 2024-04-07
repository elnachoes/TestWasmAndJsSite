use is_palindrome::is_palindrome;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("error no argument given");
    }

    println!("{}", is_palindrome(&args[1]));
}
