fn main() {
    /*
        Slices
        Permitem referenciar contiguamente elementos de uma collection, sem
        referenciar a collection inteira.

        String slices é uma referencia a uma parte de uma string:
    */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    
    // hello e world são referências a pedaços da string
    // o indice é [inicial .. final+1]
    // se inicial = 0, podemos escrever [.. final+1]
    // se final = len, podemos escrever [inicial ..] ou [inicial .. len]
    // se inicial = 0 e final = len, pode escrever [0..len] ou [..]
    // os indices sao pensados utilizando ASCII

    /*  exemplo problema: 
        Achar a primeira palavra em uma string separada por espaços, se não tiver espaços, retornar string inteira.

    */

    let frase = String::from("Ola mundo");
    let primeira_palavra = acha_primeira_palavra(&frase);
    println!("A primeira palavra de {frase} é: {primeira_palavra}");

    /*
        String literals como slices
        um string literal é um slice que aponta para o binario, &str é uma referencia
        imutável.
    */

    /*
        String slices como parâmetros
        É possível mandar como parâmetro um slice ao inves de uma referência a uma
        String.

        Quando definimos uma função para receber um slice de string, fazemos com que
        nossa API seja mais geral 
    */
    let minha_string = String::from("hello world");

    let palavra = primeira_palavra_str_literal(&minha_string[0..6]);
    let string_inteira_slice = primeira_palavra_str_literal(&minha_string[..]);

    // tambem funciona com referencias a string toda diretamente
    let string_inteira_ref = primeira_palavra_str_literal(&minha_string);

    // tambem funciona com literais

    let minha_string_literal = "hello world";
    let palavra = primeira_palavra_str_literal(&minha_string_literal[0..6]);
    let string_inteira_slice = primeira_palavra_str_literal(&minha_string_literal[..]);

    // string literals sao slices, entao isso funciona também
    let string_inteira = primeira_palavra_str_literal(&minha_string_literal);

    // OUTROS TIPOS DE SLICES
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1 .. 3]; // será igual a [2, 3]



}

fn acha_primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // retorna o slice de 0 até primeiro espaço
            return &s[0..i];
        }
    }
    // retorna a string inteira pois nao tem espaço
    return &s[..]
}

fn primeira_palavra_str_literal(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // retorna o slice de 0 até primeiro espaço
            return &s[0..i];
        }
    }
    // retorna a string inteira pois nao tem espaço
    return &s[..]
}
