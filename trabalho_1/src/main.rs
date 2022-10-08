mod atividade_1;
mod atividade_2;
use std::io;

fn main() {

    let mut n = String::new();

    println!("Escolha qual atividade quer executar:"); // 1, 2 ou 3 relativas ao pdf do trabalho 1

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");
    
    let choice:char = n.trim().parse().expect("Input not an integer");

    match choice {
        '1' => atividade_1::main(),
        '2' => atividade_2::main(),
        '3' => println!("Atividade não existente!")
    }
}