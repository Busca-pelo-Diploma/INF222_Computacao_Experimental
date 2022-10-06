use std::io;

fn main() {

    let mut n = String::new();
    
    let prob_a: f64 = 4.0/6.0;
    let prob_b: f64 = 5.0/6.0;

    println!("Escolha quantas vezes a simulação será feita!");

    io::stdin()
        .read_line(&mut n)
        .expect("Erro ao ler número desejado!");

    let number_of_simulations: i32 = n.trim().parse().expect("Input not an integer");

    println!("Serão realizadas {number_of_simulations} simulações!");

    for number in 0..number_of_simulations {
        println!("{number}");
    }

    println!("Probabilidade de A: {} - Probabilidade de B: {}", prob_a, prob_b);

}