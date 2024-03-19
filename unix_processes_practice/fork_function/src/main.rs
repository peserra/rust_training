use std::io::{stdout, Write};

use nix::libc::{self, fork, wait};
fn main() {
    unsafe {

        //cria_novos_processos()
        esperando_processos_terminarem();
    }
}

unsafe fn cria_novos_processos() {
    let id= fork();
    // cada fork duplica um novo processo, apenas o processo principal tem id != 0
    if id == 0 {
        println!("Hello from child process");
    } 
    else {
        println!("hello from parent process");
    }

}

unsafe fn esperando_processos_terminarem() {
    let id = fork();
    let n:i32;
    if id == 0 {
        n = 1;
    }
    else {
        n = 6;
    }
    if id != 0 {
        wait();
    }
    for  i in n..n+5  {
        print!("{i} ");
        stdout().flush();
    }
    println!();
}
