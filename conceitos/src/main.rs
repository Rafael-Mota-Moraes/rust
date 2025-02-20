const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

fn main() {
    // const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60; // Escopo interno, sobrescreve o escopo externo
    // Variáveis e mutabilidade
    println!("Inicio do programa");
    let x = 5;
    println!("O valor de x é: {x}");

    // x = 6; // x não é mutável, x é constante

    let x = 666;
    println!("O valor de x é: {x}");

    let mut y = 5;
    println!("O valor de y é: {y}");
    y = 6;
    println!("O valor de y é: {y}");

    println!("Uma hora tem {UMA_HORA_EM_SEGUNDOS} segundos!");
}
