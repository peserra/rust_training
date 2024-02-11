fn main() {
    /*
        Ownership:
        Conjunto de regras de como o Rust maneja memória. O principal motivo de ele existir é para manejar a informação na Heap de um jeito eficiente.

        Regras:
            - Cada valor em Ruste tem um owner.
            - Só pode haver um owner por vez.
            - Quando o owner vai fora do escopo, o valor é eliminado.
        
    */

    // ESCOPO DE VARIÁVEIS - Uma variável está no escopo do momento que ela é declarada até o final do escopo atual

    let s = "Hello"; // o escopo de s é a função main

    /* 
        O tipo String
        Dado guardado dentro da heap, mais complexo que os vistos anteriormente.
        A string 's' acima é chamada de string literal, e é imutável. Para guardar strings que nao conhecemos, ou que desejamos mudar, utilizamos o tipo String. Diferente do literal string, String é alocado na heap.
    */
    let mut outro_s = String::from("Hello");
    outro_s.push_str(", World!");

    // para lidar com esse tipo de alocação de memoria, a memória é AUTOMATICAMENTE retornada quando a variavel dona dela sai de escopo

    // EXEMPLOS MAIS COMPLEXOS
    
    // MOVE    
    let x = 5; // com inteiros, o valor de x (5) é copiado para y
    let y = x;

    /* Com strings, o funcionamento não é o mesmo. Uma string é composta de um
     ponteiro para um array na heap, sua capacidade(quantos bytes o alocador deu) e
     seu length (quantos bytes usados pela string). Essa estrutura é alocada na
     stack.
    */
    let s1 = String::from("hello");
    
    let s2 = s1; 
    // quando fazemos isso, apenas apontamos s2 para o mesmo array da heap que s1 aponta. 
    /*  como rust libera memoria depois que sai do escopo, para evitar o erro de
     liberar duas vezes a memoria (pois s1 e s2 apontam para a mesma memoria) quando
     fazemos s2 = s1, o Rust desconsidera s1, deixando-o não mais valido. Isso é
     chamado de Move (movemos s1 para s2, s1 deixou de ser valido).
    */

    // CLONE
    // É a maneira de fazer uma cópia profunda em rust, ou seja, todo o conteudo da heap será copiado.

    let s3 =  String::from("World");
    let s4  = s3.clone();
    
    // s3 e s4 são válidos

    // FUNCTIONS
    /*
        o mecanismo é parecido com passar valores para variáveis. Passar uma variável para uma função vai mover ou copiar, assim como atribuir.
    */
    let nova_string = String::from("String nova");
    pega_ownership(nova_string); // nova_string sai do escopo e nao é mais valido a partir daqui.

    let alocado_stack = 5; // alocado_stack entra no escopo
    faz_copia(alocado_stack);  // como o inteiro é alocado na stack, passar para a função só faz uma cópia

    print!("{alocado_stack}"); // é possível utilizar alocado_stack depois

    // Valores de retorno e escopo.
    /*
        Retornar valores também transferem ownerships. 
    */
    let recebe_ownership = da_ownership();
    let fornece_ownership = String::from("vai trocar ownsership");
    let pega_ownership_pela_funcao = pega_e_devolve_ownership(fornece_ownership);

    // retornando multiplos valores de uma funcao (tuplas)
    let a_ultima_string = String::from("Uma string desnecessariamente grande");
    
    let (str, len) = calcula_tamanho_string(a_ultima_string);
    println!("O tamanho da string '{str}' é {len}.");
    
}

fn pega_ownership(pega_string: String){ // pega_string entra no escopo    
    println!("{pega_string}");    
} // pega_string sai do escopo

fn faz_copia(algum_inteiro:i32){ // algum_inteiro entra no escopo
    println!("{}", algum_inteiro);
}// algum_inteiro sai do escopo

fn da_ownership() -> String {
    let alguma_string = String::from("sua string");
    return alguma_string;
}

fn pega_e_devolve_ownership(alguma_string: String) -> String {
    return alguma_string;
}

fn calcula_tamanho_string(s: String) -> (String, usize) {
    let tamanho = s.len();
    return (s, tamanho);
}