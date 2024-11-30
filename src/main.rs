// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Intersecção entre duas Retas/Vetores
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 30/11/2021
use std::io;

fn read_integer() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("falha ao ler essa linha");
    input.trim().parse().expect("Por favor, digite um número!")
}

fn intersect(vec1: &[i32], vec2: &[i32]) -> Vec<i32> {
    let mut intersection = Vec::new();
    let set1: std::collections::HashSet<_> = vec1.iter().cloned().collect();
    let set2: std::collections::HashSet<_> = vec2.iter().cloned().collect();

    for x in set1.intersection(&set2) {
        intersection.push(*x);
    }

    intersection
}

fn main() {
    println!("Insira o tamanho dos vetores a serem analisados:");
    let size = read_integer();

    let mut vec1 = Vec::with_capacity(size as usize);
    let mut vec2 = Vec::with_capacity(size as usize);

    println!("Insira os elementos do vetor 1:");
    for _ in 0..size {
        vec1.push(read_integer());
    }

    println!("Insira os elementos do vetor 2:");
    for _ in 0..size {
        vec2.push(read_integer());
    }

    let intersection = intersect(&vec1, &vec2);
    println!("Intersection: {:?}", intersection);
}