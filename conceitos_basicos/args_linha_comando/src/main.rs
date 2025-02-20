use std::env;

fn main() {
    println!("\nTotal de elementos em env::args é {}", env::args().len());

    println!("\nPercorre usando iterador:");
    let mut i = 0;
    for x in env::args() {
        println!("Argumento [{}] == {}", i, x);
        i += 1;
    }

    println!("\nPErcorre usando iterador com indices: ");
    for (i, x) in env::args().enumerate() {
        println!("Argumento [{}] == {}", i, x);
    }

    println!("\nColoca tudo em um vector");
    let argumentos: Vec<String> = env::args().collect();
    for argumento in argumentos {
        println!("Arg: [{argumento}]");
    }
}
