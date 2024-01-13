fn main() {
    /*
        Uma referencia é um ponteiro, enviamos apenas o endereço de memória. Assim podemos utilizar um valor sem ficar pegando seu ownership.
        & - operador de referencia
        * - operador de dereferencia

        REGRAS:
            - A qualquer momento, pode ter ou UMA referência mutável ou n referências
              imutáveis a um valor.
            - Referências devem ser sempre válidas
    */

    let s1 = String::from("hello");
    let len = calcula_len(&s1);
    // enviamos s1 como referencia para a função, e ainda podemos usar seu valor no print, pois nao mudamos o ownership.
    println!("O tamanho da string {s1} é igual a {len}");

    /*
        Borrowing
        É o nome dado à ação de criar uma referência. Note que, não podemos
        modificar algo que estamos pegando emprestado, temos que devolver exatamente
        como pegamos. Aqui, funciona igual, não podemos alterar um valor que pegamos
        por referência.

        A menos que o valor referenciado e o parametro recebido pela função sejam
        mutáveis, ai é possível alterar.

        É possível, porém, ter apenas UMA referência mutável a um valor, portanto,
        não é possível criar duas referências mutáveis a um mesmo valor. (nao posso
        emprestar para duas pessoas mudarem ao mesmo tempo). Isso previne 'Data 
        Races', ou seja, quando mais de um ponteiro tentam acessar uma memoria
        sem mecanismo de sincronização.
       
    */
    let mut outra_string = String::from("Eai ");
    muda_referencia(&mut outra_string); // outra string será Eai galera! ao final.

    /* 
        É possível utilizar {} para criar um novo escopo e permitir varias
        referências, porém nao simultâneas
    */
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        // quando sair do escopo, r1 deixa de existir
    }
    let r2 = &mut s;

    /*
        Também não é possível ter referências mutáveis e imutáveis simultaneamente
        para o mesmo valor em um mesmo escopo. Como o escopo acaba quando aquela
        referência não é mais usada, se tivermos um uso para as referências imutáveis
        antes da mutável, não haverá erro:
    */

    let ref1 = &s;
    let ref2 = &s;
    println!("r1: {ref1} e r2: {ref2}");
    // a partir daqui, ref1 e ref2 nao serão mais usadas

    let ref3 = &mut s; // criado após o fim do escopo de ref1 e ref2
    println!("r3: {ref3}");

    /*
        'Pendurando' referências
        Em linguagens que utilizam ponteiros, é facil criar erroneamente ponteiros
        que apontam para regiões que foram utilizadas por outros lugares. (dando free
        na memória sem limpar o ponteiro). O Rust previne esse comportamento de 
        maneira automática, garantindo que a informação não irá para fora de escopo
        antes da referência a esses dados.

        Sendo assim, se tentar criar uma função que retorna uma referência, pode dar
        problema, para evitar isso, e melhor retornar o valor.
    */

    let referencia_para_nada = pendurada();




}

// usamos o & no tipo String para indicar que é uma referencia. O parametro recebido é do "tipo" ponteiro
fn calcula_len(s: &String) -> usize {
    return s.len();
}

fn muda_referencia(s: & mut String) { 
    s.push_str("galera!");
}

fn pendurada() -> String {
    /*
        s é criado nessa função e retornado quando o escopo acaba, se estivessemos
        devolvendo &String, s iria sair do escopo e a referência seria invalida, o
        que causaria um erro
    */
    let s = String::from("pendurada");
    return s;
}
