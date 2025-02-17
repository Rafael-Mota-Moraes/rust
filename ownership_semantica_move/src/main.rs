fn main() {
    // Semântica 'copy'
    let x = 5;
    let y = x; // Valor '5' é copiado para y

    println!("y = {y}");
    println!("x = {x}");

    // Semântica copy disponível quando o tipo implementa o 'trait' Copy
    // Inteiros
    // Booleanos
    // Ponto Flutuante
    // Caracteres
    // Tuplas e arrays apenas com tipos que suportam Copy
    // (i32,bool) sim
    // (i32,String) não

    // Semântica 'move'
    let s1 = String::from("Hello");
    let s2 = s1; // s1 não é mais válida!

    println!("s2 = {s2}");
    // println!("s1 = {s1}");

    // Ainda é possível fazer um clone
    let s3 = s2.clone();
    println!("s3 = {s3}");
}
