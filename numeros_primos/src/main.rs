fn numero_primo_while(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limite = (num as f64).sqrt() as u32;
    let mut d = 2;

    while d <= limite {
        if num % d == 0 {
            return false;
        }

        d += 1
    }

    true
}

fn numero_primo_for(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limite = (num as f64).sqrt() as u32;

    for d in 2..=limite {
        if num % d == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("15 é primo? {}", numero_primo_while(15));
    println!("15 é primo? {}", numero_primo_for(15));
}
