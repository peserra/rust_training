

fn main() {
    /*
        Rust é uma linguagem estaticamente tipada

        Rust guarda os dados em diferentes tipos, há dois subconjutos de dados: Scalar e Compound

        O compilador pode, de modo geral, inferir qual o tipo de dado a variavel tem conforme seu valor, mas para operações de conversao, é necessário deixar explicito
    */
    let guess:u32 = "42".parse().expect("not a number");

    /*
        Tipos escalares (scalars)

        representam um valor unico: int, float, bool e char

        Int: podem ser unsigned ou signed, contendo diferente numero de bits:
            i8, u16, i16, u128, isize(arch) etc
            numeros negativos sao complemento de dois
        
        isize e usize dependem do tamanho da arquitetura do computador (32 ou 64 bits)

        podem ser escritos da varias formas, inclusive utilizando _ para facilitar a leitura:
            1_000; 0xff; 0o77; 0b111_000; b'A' (decimal, hex, octal, binario e byte)
         
    */
    //int
    let numero:u32 = 78; 

    //float
    let area: f32 = 65.8; //f32 (padrao é f64)

    // TIPOS COMPOSTOS
    /*
        TUPLAS

        Tipo de dado que armazena varios valores de uma vez. Temos que declarar que tipo de valores cada elemento da tupla armazenará, e acessamos cada item com seu respectivo indice.

        Uma tupla vazia é chamada Unit. O tipo e valor sao escritos () e expressoes retornam a unit se nao retornarem nenhum tipo de valor.
    */
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    
    // acessando os itens da tupla:
    let primeiroElementoTupla = tuple.0; //usamos o indice do item 

    /*
        ARRAYS

        Diferente da tupla, todos os elementos de um array devem possuir o mesmo tipo. 
        
        Arrays em rust possuem um tamanho fixo. Eles nao são flexiveis como o tipo vector.

        Vector é uma coleção similar ao array, oferecido pela biblioteca padrão, que pode mudar de tamanho.
    */
    // array generico
    let arr = [1, 2, 3, 4, 5];
    
    // array com todos os elementos iguais
    let arrayZeros = [0;10];

    let primeiroElementoArray = arr[0];


    println!("Hello, world!");
}
