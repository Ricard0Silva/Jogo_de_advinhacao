extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Bem vindo ao jogo de advinhação de numeros!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha a ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo, tente outra vez!"),
            Ordering::Greater => println!("Muito alto, tente outra vez!"),
            Ordering::Equal => {
                println!("Você acertou, Parabéns!");
                break;
            } 
        }

    }

    
}
