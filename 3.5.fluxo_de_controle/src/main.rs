fn main() {
    /*
        IF statements controlam baseado numa condicional

        

    */
    let numero = 3;

    if numero % 2 == 0{
        println!("O numero é par");
    }
    else if numero % 2 != 0{
        println!("O numero é impar");
    } else {
        println!("Voce bugou a matematica");
    }
    
    // if é uma expression, sendo assim, podemos utilizar logo depois de um let MAS os dois valores que tem possibildiade de serem o retorno do if DEVEM ser do mesmo valor, se não temos type mismatch

    let condicao = true;
    let numero = if condicao {5} else {0}; // numero recebe 5


    /*
        LOOPS

        RUST possui 3 tipos: Loop, while e for
        - loop: executa para sempre, a menos quando dito para parar.
            - pode parar com o comando do terminal ctrl+c ou com break
            - um dos usos do loop é tentar uma operação que pode falhar, como ver se uma thread terminou a operação. Alem disso, pode ser necessario passar o valor ao fim do loop, da seguinte forma:        
    */
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }; // ao final, counter = 20
    }

    // loop labels funcionam igual "alias" para especificar qual loop estamos terminando com "break" ou continuando com "continue", pois esses comandos funcionam, por padrao, de "dentro para fora"

    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}")
        let mut restante = 10;
        loop{
            println!("restante = {restante}");
            if restante == 9{
                break; // quebra o loop interno
            }

            if count == 2 {
                break 'counting_up; // quebra o loop externo
            }
            restante -= 1;
        }
        count += 1;
    }
    println!("contagem final = {count}");


    /*
        WHILE

        Funciona igual qualquer outra linguagem, o loop executa enquanto a condição for verdadeira
    */
    let mut numero_repeticoes = 10;
    while numero_repeticoes != 0 {
        println!("{numero_repeticoes}");
        numero_repeticoes -= 1;
    }

    /*
        FOR
        Melhor utilizado para iterar em uma collection, evitando erros. A sintaxe é bem parecida com python. for <iterador> in <collection>
    */
    let a = [10, 20, 30, 40, 50];
    for elemento in a {
        println!("o valor do elemento é: {elemento}");
    }

    // o exemplo do while acima pode ser escrito com for utilizando a seguinte sintaxe

    for number in (1..4).rev(){ // rev indica "reverses"
        println!("{number}");
    }

}
