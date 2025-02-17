fn main() {
    // Cria a variável imutável s1, ela fica no Stack, pois tem tamanho fixo
    //  let s1 =
    // O tipo de s1 é string slice ou uma referência para string

    // s1 aponta para um 'String literal'
    // 'String literal' é imutável, fica fixo no código

    // s1 até poderia ser mutável e apontar para outra coisa

    let s1 = "primeiro string literal";

    {
        // s2 não é válida nessa linha
        let s2 = "segundo string litera"; //  s2 é válida a partir de agora
        println!("O valor de s1 é {s1}");
        println!("O valor de s2 é {s2}");
    }

    println!("O valor de s1 é {s1}");
    // println!("O valor de s2 é {s2}"); // S2 não é válido

    // Cria a variável s3, ela fica no Stack pois tem tamanho fixo

    // Como variável do 'Tipo String' pode mudar de tamanho durante a execução, ela é alocada no Heap

    // 'String literal Alo' é copiado para a memória da varoável 'Tipo String'
    let mut s3 = String::from("Alo");

    s3.push_str(", mundo");

    // Mais memória foi alocada automaticamente para a variável 'Tipo String'
    println!("O valor de s3 agora é {}", s3);

    // s3 é dona (owns) a variável do 'Tipo String'
    // Como faço para liberar a memória usada pela variável 'Tipo String' ???

    // A memória é liberada automaticamento quando termina o escopo de seu dono

    {
        let s4 = String::from("ALO ALO ALO");
        println!("O valor de s4 é {}", s4);
    }
}
