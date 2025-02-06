fn fatorial_classico(n: i64) -> i64 {
    let mut fatorial: i64 = 1;
    for i in 2..=n {
        fatorial *= i;
    }

    fatorial
}

fn fatorial_recursivo(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    n * fatorial_recursivo(n - 1)
}

fn fatorial_iterador(n: i64) -> i64 {
    (1..=n).product()
}

fn main() {
    let fat1 = fatorial_recursivo(5);
    let fat2 = fatorial_classico(5);
    let fat3 = fatorial_iterador(5);

    println!("{fat1} {fat2} {fat3}");
}
