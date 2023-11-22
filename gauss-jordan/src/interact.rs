pub fn get_size() {
    println!("getting size")
}

pub fn create_matrix() {
    println!("creating_matrix")
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
