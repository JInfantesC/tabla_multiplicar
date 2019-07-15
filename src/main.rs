extern crate clap;

use clap::{App,Arg};

fn main() {
	let matches = App::new("Tabla de multiplicar")
		.version("0.1.0")
		.author("JInfantesC")
		.about("Muestra la tabla de multiplicar del número definido")
		.arg(Arg::with_name("numero")
			.short("n")
			.long("num")
			.takes_value(true)
			.help("Número de la tabla"))
		.arg(Arg::with_name("repeticiones")
			.short("r")
			.long("rep")
			.takes_value(true)
			.help("Número de resultados"))
		.get_matches();
	let table_of:u32 = matches.value_of("numero").unwrap_or("5").parse::<u32>().unwrap();
	let num_resultados=matches.value_of("repeticiones");
	match num_resultados{
		None => {
			printTable!(table_of);
		},
		Some(s)=>{
			let reps=s.parse::<u32>().unwrap();
			printTable!(table_of, reps);
		}
	}
}

#[macro_export]
macro_rules! printTable {
	($table_of: ident, $results: ident) => {
		for number in 1..($results+1) {
			println!(
				"{:>width$} multiplied by {:>width$} is {:>width$}",
				$table_of, number, $table_of*number, width=5);
		}
	};
	($table_of: ident) => {
		for number in 1..11 {
			println!(
				"{:>width$} multiplied by {:>width$} is {:>width$}",
				$table_of, number, $table_of*number, width=5);
		}
	};
}
