use std::io;
use rand::prelude::*;

pub fn main() {
    let mut n = String::new();

    println!("Escolha quantas vezes a simulação será feita!");

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");

    let number_of_simulations: i32 = n.trim().parse().expect("Input not an integer");

    let mut count = (0,0,0);
    let mut duration: i128 = 0;
    let mut ultrapassou = (0,0);

    for _number in 0..number_of_simulations {
    
        let mut game = (3,7,0);
        let mut winner = true;
        let mut first_place = true;

        loop {
            if game.0 == 0 {
                count.1 += 1;
                duration += game.2;
                break;
            }

            if game.1 == 0 {    
                count.0 += 1;
                duration += game.2;
                break;
            }

            if game.0 > game.1 {
                winner = false;
            }

            if first_place && game.0 > game.1 {
                first_place = false;
                ultrapassou.0 += 1;
            }

            if !first_place && game.0 < game.1 {
                first_place = true;
                ultrapassou.1 += 1;
            }

            let mut rng = rand::thread_rng();
            let number = rng.gen_range(0..2);

            if number == 0 { 
                game.0 += 1;
                game.1 -= 1;

            } else {
                game.0 -= 1;
                game.1 += 1;
            }

            game.2 += 1;
        }

        if winner == true {
            count.2+=1;
        }
    }

    let prob = (
        count.0 as f64 / number_of_simulations as f64,
        count.1 as f64 / number_of_simulations as f64,
        count.2 as f64 / number_of_simulations as f64
    );

    let ultrapassou_prob = (
        ultrapassou.0 as f64 / number_of_simulations as f64,
        ultrapassou.1 as f64 / number_of_simulations as f64,
        (ultrapassou.0 + ultrapassou.1) as f64 / number_of_simulations as f64
    );

    println!("Probabilidade de A vencer: {} - Probabilidade de B vencer: {}", prob.0, prob.1);
    println!("Duração média de uma partida: {}", duration/number_of_simulations as i128);
    println!("Probabilidade do B ficar na frente: {}", prob.2);
    println!("Média de ultrapassagem A -> B p/j: {} - Média de ultrapassagem B -> A p/j: {}", ultrapassou_prob.0, ultrapassou_prob.1);
    println!("Média de número de ultrapassagens por jogo: {}", ultrapassou_prob.2);

}