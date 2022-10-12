use std::io;
use rand::distributions::{Bernoulli, Distribution};

fn print(count: &[i32;3], count_second_strategy: &[i32;3], survival_prob: &[f64;3], survival_prob_second_strategy: &[f64;3]) {
    println!("-----------------------------------------------------------------------------");
    println!("-------------------------- Primeira Estratégia:------------------------------");
    println!("-----------------------------------------------------------------------------");
    println!("Número de Duelos Vencidos por A: {} - Probabilidade de A sobreviver: {}", count[0], survival_prob[0]);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", count[1], survival_prob[1]);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", count[2], survival_prob[2]);
    println!("-----------------------------------------------------------------------------");
    println!("--------------------------- Segunda Estratégia:------------------------------");
    println!("-----------------------------------------------------------------------------");
    println!("Número de Duelos Vencidos por A: {} - Probabilidade de A sobreviver: {}", count_second_strategy[0], survival_prob_second_strategy[0]);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", count_second_strategy[1], survival_prob_second_strategy[1]);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", count_second_strategy[2], survival_prob_second_strategy[2]);
}

pub fn main() {

    let mut n = String::new();

    let mut count = [0, 0, 0];    
    let mut count_second_strategy = [0, 0, 0];

    println!("Escolha quantas vezes a simulação será feita!");

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");

    let number_of_simulations: i32 = n.trim().parse().expect("Input not an integer");

    // First strategy - all shooters go for the highest precision marksman
    for _number in 0..number_of_simulations {

        // Each shooter precision when shooting
        let probabilities = [(4,6), (5,6), (2,6)];
        
        // Tuple of "shooters" - 0: boolean pointing is shooter is alive or not, 1: random number generator based on precision
        let mut shooters = [
            (true, Bernoulli::from_ratio(probabilities[0].0, probabilities[0].1).unwrap()),
            (true, Bernoulli::from_ratio(probabilities[1].0, probabilities[1].1).unwrap()),
            (true, Bernoulli::from_ratio(probabilities[2].0, probabilities[2].1).unwrap())
        ];
        
        loop {

            if shooters[0].0 == true {
                let shot = shooters[0].1.sample(&mut rand::thread_rng());

                if shot == true {
                    if shooters[1].0 == true { 
                        shooters[1].0 = false;
                        if shooters[2].0 == false {
                            count[0]+=1;
                            break;
                        }
                    } else if shooters[2].0 == true {
                        count[0]+=1;
                        break;
                    }
                }
            }
            
            if shooters[1].0 == true {
                let shot = shooters[1].1.sample(&mut rand::thread_rng());

                if shot == true {
                    if shooters[0].0 == true { 
                        shooters[0].0 = false;
                        if shooters[2].0 == false {
                            count[1]+=1;
                            break;
                        }
                    } else if shooters[2].0 == true {
                        count[1]+=1;
                        break;
                    }
                }
            }            
            
            if shooters[2].0 == true {
                let shot = shooters[2].1.sample(&mut rand::thread_rng());

                if shot == true {
                    if shooters[1].0 == true { 
                        shooters[1].0 = false;
                        if shooters[0].0 == false {
                            count[2]+=1;
                            break;
                        }
                    } else if shooters[0].0 == true {
                        count[2]+=1;
                        break;
                    }
                }
            }
            
        }
        
    }

    // Second estrategy - third shooters misses until one of the others is dead
    for _number in 0..number_of_simulations {

        // Each shooter precision when shooting
        let probabilities = [(4,6), (5,6), (2,6)];
        
        // Tuple of "shooters" - 0: boolean pointing is shooter is alive or not, 1: random number generator based on precision
        let mut shooters = [
            (true, Bernoulli::from_ratio(probabilities[0].0, probabilities[0].1).unwrap()),
            (true, Bernoulli::from_ratio(probabilities[1].0, probabilities[1].1).unwrap()),
            (true, Bernoulli::from_ratio(probabilities[2].0, probabilities[2].1).unwrap())
        ];
        
        loop {

            if shooters[0].0 == true {
                let shot = shooters[0].1.sample(&mut rand::thread_rng());

                if shot == true {
                    if shooters[1].0 == true { 
                        shooters[1].0 = false;
                        if shooters[2].0 == false {
                            count_second_strategy[0]+=1;
                            break;
                        }
                    } else if shooters[2].0 == true {
                        count_second_strategy[0]+=1;
                        break;
                    }
                }
            }
            
            if shooters[1].0 == true {
                let shot = shooters[1].1.sample(&mut rand::thread_rng());

                if shot == true {
                    if shooters[0].0 == true { 
                        shooters[0].0 = false;
                        if shooters[2].0 == false {
                            count_second_strategy[1]+=1;
                            break;
                        }
                    } else if shooters[2].0 == true {
                        count_second_strategy[1]+=1;
                        break;
                    }
                }
            }            
            
            if shooters[2].0 == true && (shooters[0].0 ^ shooters[1].0) {
                let shot = shooters[2].1.sample(&mut rand::thread_rng());

                if shot == true {
                    if shooters[1].0 == true { 
                        shooters[1].0 = false;
                        if shooters[0].0 == false {
                            count_second_strategy[2]+=1;
                            break;
                        }
                    } else if shooters[0].0 == true {
                        count_second_strategy[2]+=1;
                        break;
                    }
                }
            }
            
        }
        
    }
    
    
    let survival_prob = [
        (count[0] as f64) / (number_of_simulations as f64),
        (count[1] as f64) / (number_of_simulations as f64),
        (count[2] as f64) / (number_of_simulations as f64),
    ];

    let survival_prob_second_strategy = [
        (count_second_strategy[0] as f64) / (number_of_simulations as f64),
        (count_second_strategy[1] as f64) / (number_of_simulations as f64),
        (count_second_strategy[2] as f64) / (number_of_simulations as f64)
    ];

    print(&count, &count_second_strategy, &survival_prob, &survival_prob_second_strategy);

}