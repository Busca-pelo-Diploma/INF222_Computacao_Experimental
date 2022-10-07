use std::io;
use rand::distributions::{Bernoulli, Distribution};

fn main() {

    let mut n = String::new();

    let mut count_a = 0;    
    let mut count_b = 0;

    println!("Escolha quantas vezes a simulação será feita!");

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");

    let number_of_simulations: i32 = n.trim().parse().expect("Input not an integer");

    for _number in 0..number_of_simulations {

        let mut duel_ended = false;

        let probability_a: f64 = 4.0/6.0;
        let probability_b: f64 = 5.0/6.0;

        let mut probability_a_aux: f64 = 4.0/6.0;
        let mut probability_b_aux: f64 = (5.0/6.0)*(1.0-probability_a);

        let mut prob_a = Bernoulli::new(probability_a).unwrap();
        let mut prob_b = Bernoulli::new(probability_b).unwrap();

        while !duel_ended {            
            let value_a = prob_a.sample(&mut rand::thread_rng());

            if value_a != true {
                
                probability_a_aux = probability_a_aux * (1.0-probability_a) * (1.0-probability_b);
                prob_a = Bernoulli::new(probability_a).unwrap();

                let value_b = prob_b.sample(&mut rand::thread_rng()); 
                
                if !value_b {
                    probability_b_aux = probability_b_aux * (1.0-probability_a) * (1.0-probability_b);
                    prob_b = Bernoulli::new(probability_b).unwrap();
                } else {
                    count_b+=1;
                    duel_ended = true;
                }
                
                
            } else {
                count_a+=1;
                duel_ended = true;
            }        
        }        
    }

    let a_survival_prob: f64 = (count_a as f64) / (number_of_simulations as f64);
    let b_survival_prob: f64 = (count_b as f64) / (number_of_simulations as f64);

    println!("-------------------------------------------------------------------------------------------------");
    println!("Número de Duelos Vencidos por A: {} - Probabilidade de A sobreviver: {}", count_a, a_survival_prob);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", count_b, b_survival_prob);
    println!("-------------------------------------------------------------------------------------------------");

}