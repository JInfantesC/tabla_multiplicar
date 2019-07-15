# Tabla de multiplicar [RUST]
Programa que te muestra la tabla de multiplicar del número que el usuario indique.
## Objetivo
Hacer una pequeña aplicación para aprender conceptos de Rust, Cargo, la gestión de paquetes y macros.

El programa usa el paquete [Clap] para manejar los argumentos y una macro para imprimir la tabla.

## Uso
```bash
$ cargo run -- --help
Tabla de multiplicar 0.1.0
JInfantesC
Muestra la tabla de multiplicar del número definido

USAGE:
    tabla_multiplicar [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --num <numero>          Número de la tabla
    -r, --rep <repeticiones>    Número de resultados

$ cargo run -- -n 5 -r 4
5 multiplied by     1 is     5
5 multiplied by     2 is    10
5 multiplied by     3 is    15
5 multiplied by     4 is    20
```

## Enlaces
- Argument Parsing - Rust Cookbook https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
- Clap https://docs.rs/clap/2.33.0/clap/
- Designators - Rust By Example https://doc.rust-lang.org/rust-by-example/macros/designators.html
- https://stackoverflow.com/questions/24047686/default-function-arguments-in-rust#35369909
