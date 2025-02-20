// Toda variável tem um escopo
// Ela é válida somente dentro daquele escopo
// Escopo é definido pelo momento da definição de destruição

fn exemplo() {
    let s1 = "Primeiro string literal"; // s1 é válida a partir de agora

    {
        // s2 não é válida nessa linha
        let s2 = "Segundo string literal"; // s2 é válida a partir de agora
        println!("Valor de s1 é {s1}");
        println!("Valor de s2 é {s2}");
    }

    // s2 não existe mais
    println!("O valor de s1 é {s1}");
}

fn main() {
    exemplo();
    println!("Hello, world!");
}
