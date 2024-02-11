/*
    Structs
    Sao um tipo de dado que permite agrupar e nomear multiplos dados relacionados que compõe um grupo que faça sentido estar junto. 
    São mais flexiveis que tuplas, nao sendo necessário inserir os dados na ordem que foi especificado, como na tupla.

    Sao expressions, logo a mesma regra de expressions funcionam para a struct

    Para criar uma struct, usamos a keyword struct <nome> {} 
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn cria_usuario(email: String, username: String) -> User {
    // struct é retornada como ultima expression da função
    User {
        active: true,
        // como tem o mesmo nome q os parametros da função, é possivel o abaixo também
        username,
        email: email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct SempreIgual;
fn main() {
    // instanciando uma struct
    /*
        Adicionamos os valores na ordem que quisermos no modelo chave:valor, de modo que a struct funciona como um template e as instancias como a criação concreta desse template (tipo objetos).
    */
    let mut user1 = User {
        active: true, 
        username: String::from("umusuario1"), 
        email: String::from("umusuairo@empresa.com"), 
        sign_in_count: 1,
    };

    /*
        Para acessar um valor dessa struct, usamos o . para especificar qual queremos. Se a instancia for mutável, podemos alterar os valores dessa forma também. 
    */
    user1.email = String::from("outrousuario@empresa.com");

    cria_usuario(
        String::from("joaoAlmeida"), 
        String::from("joaoAlmeida.com") 
    );

    /*
        Se quisermos instanciar uma outra struct que tenha quase todos os valores iguais, exceto alguns, podemos utilizar a sintaxe Struct Update 
        '..<struct origem>'

        Ela especifica que, "tudo não especificado é igual à outra struct"
    */

    let user2 = User {
        email: String::from("outro@exemplo.com"),
        ..user1 //todo o resto das infos será igual à user1
    };
    /*
        IMPORTANTE
        Ao fazer isso, não será mais possivel utilizar todos os valores de user1, pois a string username teve seu valor movido para user2. Se tivessemos especificado o username "manualmente", conseguiriamos utilizar ambas as structs, pois os outros valores (active e sign_in) são apenas copiados e nao movidos (mesma regra ja vista antes).
    */
  

    /* 
        TUPLE STRUCTS
        Elas garantem a possibilidade de dar nome às tuplas, sem precisar adicionar  o  nome dos campos como numa struct
    */
    let preto = Color(0,0,0);
    let triangulo = Point(0,4,3);

    /*
        Unit structs
        São structs que nao possuem valores, utilizadas quando queremos implementar alguma trait ou algum tipo mas não queremos guardar nenhum valor nesse tipo.

        a declaração é struct <nome> ;
    */

    let assunto = SempreIgual; // mais sobre isso mais pra frente.
    

}
