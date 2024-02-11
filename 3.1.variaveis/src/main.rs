fn main() {
    let mut x = 5; // toda variável em rust é imutável, a menos que explicitado.
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é: {x}");

    /*
        Constantes
        
        constantes nao podem ser mutaveis e devem ter seu tipo explicitado,
        além disso, elas podem ser atribuidas apenas a uma expressão constante, e não
        ao resultado de uma operação computada em tempo de execução.
        Constantes sao validas durante todo o tempo que o escopo que elas foram declaradas.
    */ 
    const  PI: f32 = 3.14;// diferenca entre constantes e variaveis


    /*
        Shadowing

        É o conceito de esconder o valor de uma variavel em tempo de execução, temporariamente, criando uma variavel com o mesmo nome. O shadowing termina quando acaba o escopo em que ela foi declarada, utilizado declarando entre {}
     */

    let y = 5;
    let y = y + 1;

    {
        // aqui o y esta "shadowing" o valor anterior de y ate  o fim do escopo
        let y = y * 2;
        println!("O valor de y nesse escopo interno é {y}");
    }
    println!("O valor de y no escopo externo é {y}");

    
}
