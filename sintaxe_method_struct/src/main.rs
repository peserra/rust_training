/*
    Metodos sao igual funções, mas usados dentro do contexto de structs, enums e traits.

    A diferença é que o primeiro parametro de um metodo é SEMPRE self

    É possível que o nome do método seja o mesmo de um elemento da struct, o Rust sabe que queremos acessar o método se colocarmos <nome>()
*/

struct Triangulo{
    altura: u32,
    base: u32,
}

struct Retangulo {
    altura: u32,
    base: u32,
}

// é permitido que haja multiplos blocos de implementação por struct
impl Triangulo {

    // se quisessemos mudar a instancia que chama esse metodo, usariamos &mut self
    // utilizar apenas self e pegar ownership e raro, só quando quisermos que nao seja possivel para o usuario pegar a struct original.
    fn area(&self) -> u32 {
        return self.altura * self.base / 2;
    }

    // utilizando mais de um parâmetro num metodo
    fn consegue_conter(&self, outro: &Triangulo) -> bool {
        return self.area() > outro.area();
    }
}

impl Retangulo {
    // Associated functions funcionam como construtores, nao sao metodos pois nao tem parametro self, eles retornam uma nova instancia da struct (tipo o new).
    fn quadrado(comprimento: u32) -> Self {
        Self{ altura: comprimento, base: comprimento}
    }
}

fn main() {
    let tr1 = Triangulo {altura: 3, base: 4,};
    let tr2 = Triangulo{altura: 4, base: 4,};
    println!("a área do triangulo é: {}", tr1.area());
    println!(" o Triangulo 1 consegue conter o 2 ? {}", tr1.consegue_conter(&tr2));
    println!(" o Triangulo 2 consegue conter o 1 ? {}", tr2.consegue_conter(&tr1));

    // para chamar a funcao associada e criar instancia de quadrado: 
    let quadrado = Retangulo::quadrado(4);

}
