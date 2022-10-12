use std::io;
use rand::distributions::{Bernoulli, Distribution};

fn print(count_a: &i32, count_b: &i32, survival_prob: &[f64;2]) {
    println!("-------------------------------------------------------------------------------------------------");
    println!("Número de Duelos Vencidos por A: {} - Probabilidade de A sobreviver: {}", *count_a, survival_prob[0]);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", *count_b, survival_prob[1]);
    println!("-------------------------------------------------------------------------------------------------");
}

pub fn main() {

    let mut n = String::new();

    let mut count_a = 0;    
    let mut count_b = 0;

    println!("Escolha quantas vezes a simulação será feita!");

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");

    let number_of_simulations: i32 = n.trim().parse().expect("Input not an integer");

    for _number in 0..number_of_simulations {

        let probability = [(4,6),(5,6)];

        let generators = [
            Bernoulli::from_ratio(probability[0].0, probability[0].1).unwrap(),
            Bernoulli::from_ratio(probability[1].0, probability[1].1).unwrap()
        ];

        loop {
                        
            let value_a = generators[0].sample(&mut rand::thread_rng());

            if !value_a {                            

                let value_b = generators[1].sample(&mut rand::thread_rng()); 

                if value_b {
                    count_b+=1;
                    break;
                }

            } else {
                count_a+=1;
                break;
            }        
        }        
    }

    let survival_prob = [(count_a as f64) / (number_of_simulations as f64), (count_b as f64) / (number_of_simulations as f64)];

    print(&count_a, &count_b, &survival_prob);

}