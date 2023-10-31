// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let x = 0;
	print_type_of(&x);
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
