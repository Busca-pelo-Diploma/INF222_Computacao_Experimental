mod atividade_1;
mod atividade_2;
mod atividade_3;
use std::io;

fn main() {

    let mut n = String::new();

    println!("Escolha qual atividade quer executar: (1,2 ou 3)"); // 1, 2 ou 3 relativas ao pdf do trabalho 1

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");
    
    let choice:i32 = n.trim().parse().expect("Input not an integer");

    match choice {
        1 => atividade_1::main(),
        2 => atividade_2::main(),
        3 => atividade_3::main(),
        _ => println!("Atividade não existente! Execução encerrada!")
    }
}