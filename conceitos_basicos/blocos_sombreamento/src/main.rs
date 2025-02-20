fn main() {
    println!("Início do programa");
    const X: i32 = 5;
    let y: i32 = 6;
    let mut z = 7;
    z += 1;

    println!("No início os valores são: x={X}, y={y}, z={z}");

    {
        // Bloco interno
        const X: i32 = 555;
        let y = 666;
        let mut z = 777;
        z += 1;
        println!("Dentro do bloco interno os valores são: x={X}, y={y}, z={z}");
    }
    println!("Depois do bloco interno os valores são: x={X}, y={y}, z={z}");

    println!("Sombreamento");

    let x = 5;
    println!("O valor de x é: {x}");
    let x = x + 1;
    println!("O valor de x é: {x}");

    {
        let x = x * 2;
        println!("O valor de x no do bloco interno é: {x}");
    }
    println!("O valor de x depois do bloco interno é: {x}");

    let spaces = "    ";
    let spaces = spaces.len(); // let cria nova variável com novo tipo
    println!("O valor de spaces é: {spaces}");

    let mut spaces2 = "    ";
    println!("O valor de spaces2 é: {spaces2}");
    spaces2 = "qwerty";
    println!("O valor de spaces2 é: {spaces2}");
}
