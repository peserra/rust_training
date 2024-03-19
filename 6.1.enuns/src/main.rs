/*
    Uma maneira prática de dizer que um valor pode ser algum dentro de um conjunto de valores. Por exemplo, um Retangulo pode ser um valor possível dentro de uma Forma, assim como Circulo e Triangulo.

    Como o próprio nome diz, é uma maneira de enumerar os valores possíveis para determinado value assumir.

    um enum pode ser apenas um dos seus possíveis valores (mutuamente excludente).

    Podemos colocar qualquer tipo de dado dentro de uma variante de um mesmo enum, inclusive outro enum.
*/

// enum que determina qual tipo de endereço de ip é possível.
enum tipoEndIP{
    // cada variante do enum se torna uma instancia construida dessa variante.
    V4(u8, u8, u8, u8),// serao associados 4 ints a esse valor 
    V6(String),// determina que serão associados strings a esse tipo de valor 
}

// enum de mensagem em que cada variante guarda diferentes tipos e quantidades de dados.
enum Mensagem {
    Sair,
    Movimento { x: i32, y: i32 },
    Escrever(String),
    MudaCor(i32, i32, i32),
}

// assim como structs, é possível definir metodos para enuns
impl Mensagem {
    fn call(&self) {
        // definir corpo aqui
    }
}

/*
    OPTION ENUM
    Ele atende à situação em que um valor pode ser alguma coisa ou nada.

    Em rust, null é um valor que indica que não há valor, e usando o enum é possível evitar vários bugs que podem acontecer (chamar um valor quando está null, ou vice versa).

    Rust nao possue a propriedade null, mas tem um enum que encoda o conceito de um valor estar presente ou ausente, esse enum é o Option<T>. Não é necessário explicitamente dizer Option:: para chamar None ou Some.

    <T> é uma sintaxe que indica um parâmetro de tipo genérico. Ou seja, a variante Some do Enum pode ser de qualquer valor, e qualquer "instancia" de Some terá o tipo que Some tiver.

    A vantagem de ter o Option<T> ao inves de um null, é que o compilador não nos deixa realizar operações entre um Option<T> e um T, pois estes são tipos diferentes. Sendo assim, evita-se o erro de se operar um valor nulo, visto que é necessário converter Option<T> para T, pois estes são tipos distintos.

    para se ter um value que pode ser null, deve-se obrigatoriamente ser um Option<T>, e alem disso, tratar sempre que for null. Sempre que o tipo não for um Option<T> É SEGURO ASSUMIR QUE NAO É NULO.

    enum possue uma vasta quantidade de metodos para que se possa utilizar o valor <T> do Option. Alem disso, o match é uma forma de fluxo de controle que utiliza essa ideia de Some e None
*/

enum Option<T> {
    Some(T),
    None, 
}


fn main() {
    // criar instâncias diferentes do enum, ambas as instâncias são do mesmo tipo 'tipoEndIp'
    let casa = tipoEndIP::V4(192, 168, 0, 1);
    let loopback = tipoEndIP::V6(String::from("::1"));

    let m = Mensagem::Escrever(String::from("hello"));

    // são de tipos diferentes
    let algum_char = Some('c');
    let algum_numero = Some(5);
    m.call();
}
