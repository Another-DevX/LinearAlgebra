mod interact;
use interact::print_matrix;

fn main() {
    let mut matrix: Vec<Vec<f32>> = vec![
        vec![2., 4., 6., 18.],
        vec![4., 5., 6., 24.],
        vec![3., 1., -2., 4.],
    ];
    print_matrix(&matrix);
    let matrix = jordan_gauss_elimination(matrix);
    print_matrix(&matrix);
}

fn jordan_gauss_elimination(mut matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {

    let n_rows = matrix.len();
    let m_cols = matrix[0].len();

    for diagonal in 0..n_rows {
        print_matrix(&matrix);
        if matrix[diagonal][diagonal] != 1. {
            matrix = pivote(matrix, diagonal, diagonal);
        }
        for i in 0..n_rows {
            if i != diagonal {
                let factor = matrix[i][diagonal];
                for j in 0..m_cols {
                    matrix[i][j] -= matrix[diagonal][j]*factor ;
                }
            }
        }
    }
    matrix
}

fn pivote(mut matrix: Vec<Vec<f32>>, i: usize, j: usize) -> Vec<Vec<f32>> {
    let divisor = matrix[i][j];
    for k in 0..matrix[i].len() {
        if matrix[i][k] == 0. {
            continue;
        }
        matrix[i][k] = matrix[i][k] / divisor;
    }
    matrix
}

// matrix[i][j] = matrix[i][j] - matrix[0][j];
