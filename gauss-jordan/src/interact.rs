use std::io::{self};

pub fn create_matrix() -> Vec<Vec<f32>> {
    println!("Ingrese el tamaño de la matriz (n x n):");
    let mut size_str = String::new();
    io::stdin().read_line(&mut size_str).unwrap();
    let size: usize = size_str
        .trim()
        .parse()
        .expect("Por favor ingrese un número válido");

    let mut matrix = vec![vec![0.0; size]; size];

    for i in 0..size {
        println!("Ingrese la fila {} (separada por espacios), recuerde añadir el valor de la matriz extendida:", i + 1);
        let mut row_str = String::new();
        io::stdin().read_line(&mut row_str).unwrap();

        matrix[i] = row_str
            .trim()
            .split_whitespace()
            .map(|num| num.parse().expect("Por favor ingrese números válidos"))
            .collect::<Vec<f32>>();

        if matrix[i].len() != size + 1 {
            panic!("Número incorrecto de elementos en la fila {}", i + 1);
        }
    }

    println!("Matriz ingresada:");
    print_matrix(&matrix);
    matrix
}

pub fn print_matrix(matrix: &Vec<Vec<f32>>) {
    for row in matrix {
        for (i, val) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("| {:5} ", val);
                continue;
            }
            print!("{:5} ", val);
        }
        println!();
    }
    println!();
}
