use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len()>1{
		let table_of:u32=args[1].parse::<u32>().unwrap();
		for number in 1..11 {
			println!(
				"{:>width$} multiplied by {:>width$} is {:>width$}",
				table_of, number, table_of*number, width=5);
		}
	}else{
		println!("Please, give me a number")
	}
}
