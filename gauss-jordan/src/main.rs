mod interact;
use interact::print_matrix;

fn main() {
    let mut matrix: Vec<Vec<f32>> = vec![
        vec![2., 4., 6., 18.],
        vec![2., 5., 6., 24.],
        vec![3., 1., -2., 4.],
    ];
    print_matrix(&matrix);
    println!();
    let matrix = jordan_gauss_elimination(matrix);
    print_matrix(&matrix);
}

fn jordan_gauss_elimination(mut matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    // Hallar pivote, en caso de que pivote distinto 1, dividir toda la fila
    // Hallar divisores para hacer ceros en la columna del pivote
    // Aplicar operaciones elementales a cada fila

    let n_rows = matrix.len();
    let m_cols = matrix[0].len();

    if matrix[0][0] != 1. {
        let divisor = matrix[0][0];
        for i in 0..n_rows {
            matrix[0][i] = matrix[0][i] / divisor;
        }
    }

    matrix
}
