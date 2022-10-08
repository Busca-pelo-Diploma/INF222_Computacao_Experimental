use std::io;
use rand::distributions::{Bernoulli, Distribution};

fn print_result(count_a: &i32, count_b: &i32, a_survival_prob: &f64, b_survival_prob: &f64) {
    println!("-------------------------------------------------------------------------------------------------");
    println!("Número de Duelos Vencidos por A: {} - Probabilidade de A sobreviver: {}", *count_a, *a_survival_prob);
    println!("Número de Duelos Vencidos por B: {} - Probabilidade de B sobreviver: {}", *count_b, *b_survival_prob);
    println!("-------------------------------------------------------------------------------------------------");
}

fn update_probability_values(value: &mut f64, generator: &mut Bernoulli, probability_a: f64 , probability_b: f64) {
    (*value) = (*value) * (1.0-probability_a) * (1.0-probability_b);
    (*generator) = Bernoulli::new(*value).unwrap();
}

pub fn main() {

    let mut n = String::new();

    let mut count_a = 0;    
    let mut count_b = 0;
    let mut count_c = 0;

    println!("Escolha quantas vezes a simulação será feita!");

    io::stdin().read_line(&mut n).expect("Erro ao ler número desejado!");

    let number_of_simulations: i32 = n.trim().parse().expect("Input not an integer");

    for _number in 0..number_of_simulations {        

        // Each shooter precision when shooting
        let probabilities = [4.0/6.0, 5.0/6.0, 2.0/6.0];

        for i in 0..1 {
            
            // Tuple of "shooters" - 0: boolean pointing is shooter is alive or not, 1: random number generator based on precision
            let mut shooters = [
                (true, Bernoulli::new(probabilities[0]).unwrap()),
                (true, Bernoulli::new(probabilities[1]).unwrap()),
                (true, Bernoulli::new(probabilities[2]).unwrap())
            ];
            
            let mut duel_ended = false;
            
            let mut probability_a_aux: f64 = probabilities[0];
            let mut probability_b_aux: f64 = probabilities[1]*(1.0-probabilities[0]);
            let mut probability_b_aux: f64 = probabilities[2]*(1.0-probabilities[0])*(1.0-probabilities[1]);
        }
    }        

}