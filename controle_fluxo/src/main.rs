fn main() {
    let number = 3;

    if number < 5 {
        println!("Condição verdadeira");
    } else {
        println!("Condição falsa");
    }

    if number % 4 == 0 {
        println!("número é divisivel por 4");
    } else if number % 3 == 0 {
        println!("número é divisivel por 3");
    } else if number % 2 == 0 {
        println!("número é divisivel por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }

    // Como expressão.

    let outro = if number == 0 { 0 } else { 99 };
    println!("Outro number: {outro}");

    // Estruturas de repetição

    let mut number = 3;
    println!("Usando while");
    while number != 0 {
        println!("while {number}");
        number -= 1;
    }

    println!("usando for");
    let vetor = [10, 20, 30, 40, 50];

    for elemento in vetor {
        println!("Elemento: {elemento}");
    }

    for number in 1..=4 {
        println!("com = {number}");
    }

    let mut i = 0;

    loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("i {i}");
        if i >= 10 {
            break;
        }
    }

    let result = loop {
        i += 100;
        if i >= 100 {
            break i * 2;
        }
    };

    println!("result {result}");
}
