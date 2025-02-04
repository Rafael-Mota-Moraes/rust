fn main() {
    // AS TUPLAS...

    let tup1: (i32, f64, bool) = (500, 6.4, true);

    let tup2 = (500, 6.4, true);

    println!("Minha tupla tem: {:?} {:?}", tup1, tup2);

    // desestruturação quebra a tupla em suas partes
    let (x1, y1, z1) = tup1;

    println!("Minha tupla tem: {x1} {y1} {z1}");

    // Pode acessar os campos usando indexadores
    println!("Minha tupla tem: {:?} {:?} {:?}", tup1.0, tup1.1, tup1.2);

    // Uma tupla vazia é chamada de unit, representa um valor vazio
    println!("Tupla vazia: {:?}", ());

    // Arrays ou vetores
    let aa = [1, 2, 3, 4, 5];
    let meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Maio",
        "Abril",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
    ];

    let bb: [i32; 5] = [3; 5];

    let cc = [3; 5];
    let dd = [3, 5];

    println!("cc {:?}", cc);
    println!("dd {:?}", dd);

    // Indexa começando pelo elemento 0
    println!("Elemento 2 do array 'meses' é: {:?}", meses[2]);
}
