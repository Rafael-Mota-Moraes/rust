fn main() {
    let palavra = String::from("abacaxi");

    let len1 = calcula_tamanho_move(palavra.clone());
    println!("O tamanho1 de '{}' é {}.", palavra, len1);

    let len2 = calcula_tamanho_referencia(&palavra);
    println!("O tamanho2 de '{}' é {}.", palavra, len2);

    // É a mesma coisa que na linguagem C?
    let x = 11;
    soma_900(&x);
    soma_900(&22);

    let s = String::from("hello");
    change1(&s);

    let mut x = String::from("hello");
    change2(&mut x);
    println!("Valor de x {}", x);
}

// A propriedade do 'String' é recebida pela função
fn calcula_tamanho_move(s: String) -> usize {
    s.len()
}
// s fica inválido
// s tinha a propriedade do 'String', drop do 'String'

// Um empréstimo do 'String' é recebido pela função, e não a propriedade do 'String'
fn calcula_tamanho_referencia(s: &String) -> usize {
    s.len()
}

// Referência (empréstimo) não é a mesma coisa que endereço '&' em C

fn soma_900(ref_int: &i32) {
    let c_a = *ref_int + 900;
    let s_a = ref_int + 900;

    println!("Com asterisco {} sem asterisco {}", c_a, s_a);
}

// some_string é imutável
fn change1(some_string: &String) {
    // some_string.push_str(", world!"); // Imutável
}

// some_string é mutável
fn change2(some_string: &mut String) {
    some_string.push_str(", world!");
}
