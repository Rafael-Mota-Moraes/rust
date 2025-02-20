fn outra_funcao() {
    println!("Outra função");
}

fn outra_funcao_com_parametro(x: i32) {
    println!("Outra função recebeu {x}.")
}

fn print_labeled_measurement(valor: f64, unidade: char) {
    println!("A medida é: {valor}{unidade}.")
}

fn soma(x: i32, y: i32) -> i32 {
    x + y
}

fn somaret(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("Hello, world!");
    outra_funcao();
    outra_funcao_com_parametro(10);
    print_labeled_measurement(123.4, 'm');

    let xy = soma(3, 4);
    println!("Somas deram {} e {}", xy, somaret(3, 4));
}
