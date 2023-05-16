extern crate core;

use std::fs::File;
use std::io::Write;
use rand::Rng;
use std::io::{BufRead, BufReader};

fn generate_number(len: usize, file: &mut File) {
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        file.write_all(rng.gen::<i32>().to_string().as_bytes()).expect("Impossible d'insérer le nombre généré");
        file.write_all(b"\n").expect("Impossible d'ajouter une nouvelle ligne");
    }
    file.flush().expect("Impossible de vider le tampon d'écriture");
}

fn calculate_sum_from_file(file_path: &str) -> Result<u32, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        for number_str in line.split_whitespace() {
            if let Ok(number) = number_str.parse::<u32>() {
                sum += number;
            }
        }
    }

    Ok(sum)
}

fn main() {
    // Generate var
    let number_to_generate: i32 = 2;
    let file_name: &str = "nombres.txt";

    // Creation of file
    println!("Création du fichier {}...", file_name);
    let mut file = File::create(file_name).expect("Cannot create file");

    // Add numbers to the file
    println!("Génération de {} nombres...", number_to_generate);
    generate_number(number_to_generate as usize, &mut file);

    // get result of sum of all number with the function get_sum_numbers
    match calculate_sum_from_file(file_name) {
        Ok(sum) => println!("Somme des nombres : {}", sum),
        Err(err) => eprintln!("Erreur : {}", err),
    }
}