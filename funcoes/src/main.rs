fn outra_funcao(x: i32, y:char){
    println!("Olá da outra função\n O primeiro parâmetro é {x}, o segundo é {y}");
}

fn funcao_com_retorno(numero: i32) -> i32{
    /*
        Precisamos especificar o tipo de retorno com o simbolo ->

        O valor de retorno de uma função é sempre o mesmo da ultima expressao dentro do escopo dessa função. O comando return pode ser usado para retornar antes, porem por padrão o comportamento é o supracitado
    */
    if numero == 2{
        return 10; // retorno antes
    }

    5
}

fn main() {
   /*
        Funcoes:

        Usam a convenção "snake_case" para nomes

        todos os parametros de função devem ter seu tipo explicitado
   */
    outra_funcao(5, 'h');

    /*
        Statements e Expressions

        o corpo de funções em rust é constituido de uma serie de statements que podem ou nao terminar em uma expression

        Statements: instruções que performam ações e não retornam valores
            - Definir variaveis
        Expressions: avaliam para um valor resultante
            - chamar uma função ou macro
            - criar um novo bloco de escopo
        
        Adicionar um ";" ao final de uma linha transforma em um statement. Expressões nao tem um ponto e virugla ao final


    */
    let retorno = funcao_com_retorno(3);
    println!("Valor retornado pela funcao: {retorno}");
}
