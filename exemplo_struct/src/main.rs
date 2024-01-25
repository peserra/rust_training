/*
    Um codigo de exemplo que explicita a usabilidade de uma struct, tanto na clareza dos dados utilizados como para definir os elementos em si.

    Codigo deverá calcular a área de um retangulo.
*/
#[derive(Debug)] // para debugar na macro println!
struct Retangulo {
    altura: u32,
    largura: u32,
}

fn main() {
    let retangulo = Retangulo {
        altura:5,
        largura:10,
    };
    let area = calcula_area(&retangulo);
    println!("A área do retangulo é {area}");
    println!("A instância da struct é {:?}", retangulo); // para debugar a instancia da struct
}

// pegamos a referência à struct para manter ownership na main
fn calcula_area (retangulo: &Retangulo) -> u32 {
    // struct garante mais clareza no que são os dados
    return retangulo.altura * retangulo.largura;
}
