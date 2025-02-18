fn main() {
    let s = String::from("hello");
    recebe_ownership(s); // Propriedade do 'String' entregue para a função s não é mais válido

    // println!("{}", s);

    let x = 5;
    recebe_copia(x); // Apenas copiado para a função
                     // x continua válido

    println!("{}", x);

    let s1 = devolve_ownership();
    let s2 = String::from("hello");
    let s3 = recebe_e_devolve_ownership(s2.clone());

    println!("s1 - {}. s2 - {}. s3 - {}", s1, s2, s3);
} // s não é dono do String, então ele não é liberado da memória neste ponto

fn recebe_ownership(um_string: String) {
    println!("{}", um_string);
}

/*
um_string sai fora de escopo
um_string fica inválido
ele é dono do 'String'
a memória do 'String' é liberada (drop)
*/

fn recebe_copia(um_inteiro: i32) {
    println!("{}", um_inteiro);
}

/*
um_inteiro sai fora de escopo
um_inteiro fica inválido
ele não é dono de ninguém, nenhum drop acontece
*/

fn devolve_ownership() -> String {
    let algo = String::from("aaa");
    algo
}

fn recebe_e_devolve_ownership(um_string: String) -> String {
    println!("{}", um_string);
    um_string
}
