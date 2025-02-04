// Velocidade máxima de qualquer veículo em metros por segundo
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);

// Comprimento máximo de qualquer veículo em metros
const COMPRIMENTO_MAXIMO: i32 = 22;

/*
Tipos inteiros em Rust

Length   Signed  Unsigned
8-bit    i8      u8
16-bit   i16     u16
32-bit   i32     u32
64-bit   i64     u64
arch     isize   usize

Obs1: i32 é default para inteiros
Obs2: Em caso de overflow temos "pânico na execução" (em debug mode) ou "dá a volta (release mode)"
Obs3: Existem vários métodos na biblioteca padrão para lidar com overflow
Obs4: Existem dois tipos de ponto flutuante: f32 e f64 (default)
*/

/*
Tipos literais em Rust
Numero liteal             Exemplo
Decimal                   98_222
Hex                       0xff
Octal                     0o77
Binario                   0b1111_0000
Byte (u8 apenas)          b'A'

Obs1: Ponto flutuante aceita 7.6e-2 ou 0.-76

*/

fn main() {
    let chassi: i32 = 123456; // identificação de um carro
    let acel_max: f64 = 3.0; // metros por segundo ao quadrado
    let acel_min: f64 = -10.0; // metros por segundo ao quadrado
    let vel_max: f32 = VELOCIDADE_MAXIMA as f32; // metros por segundo
    let comprimento: i32 = 4; // metros
    let posicao_atual: f64 = -100.0; // metros do cruzamento
    let vel_atual: f64 = 0.0; // metros por segundo
    let acel_atual: f64 = 0.0; // metros por segundo ao quadrado

    // adição
    let sum = posicao_atual + 10.0;

    // subtração
    let difference = vel_atual - 4.3;

    // multiplicação
    let product = comprimento * 2;

    // divisão
    let quotient = acel_atual / 2.0;
    let floored = 2 / 3;

    // resto da divisão
    let remainder = 43 / 5;

    // transdormação de tipos
    let xxx: f64 = 1235 as f64;
    let yyy = xxx + 88f64;
    // let yyy = xxx + 88 as f64;

    println!(
        "trunc {}, round {}, ceil {}, floor {}",
        xxx.trunc(),
        xxx.round(),
        xxx.ceil(),
        xxx.floor()
    );

    let t = true;
    let f = false;

    let x = t && f;
    let y = t || !f;
    let z = 12 > 13;
    let c = 'z';
    let _c = 'z'; // _ elimina o warning

    println!("bool: {x}, char {c}")
}
