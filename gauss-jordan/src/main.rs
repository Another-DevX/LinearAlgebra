mod interact;
use interact::print_matrix;

fn main() {
    let mut matrix: Vec<Vec<f32>> = vec![
        vec![1., 2., 3., 4.],
        vec![2., 4., 6., 8.],
        vec![3., 5., 7., 9.],
        vec![6., 10., 14., 20.],
    ];
    print_matrix(&matrix);
    let matrix = jordan_gauss_elimination(matrix);
    print_matrix(&matrix);
}

fn jordan_gauss_elimination(mut matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n_rows = matrix.len();
    let m_cols = matrix[0].len();

    for diagonal in 0..n_rows {
        if matrix[diagonal][diagonal] == 0. {
            swap_with_nonzero_row(&mut matrix, diagonal);
        }
        print_matrix(&matrix);
        if matrix[diagonal][diagonal] != 1. {
            matrix = pivote(matrix, diagonal, diagonal);
        }
        for i in 0..n_rows {
            let mut degenerated = true;
            if i != diagonal {
                let factor = matrix[i][diagonal];
                for j in 0..m_cols {
                    if matrix[i][j] != 0. && j == m_cols - 1 {
                        panic!("inconsistent system")
                    } else if matrix[i][j] != 0. {
                        degenerated = false;
                    }
                    matrix[i][j] -= matrix[diagonal][j] * factor;
                }
            }
        }
    }
    matrix.sort_by(|a, b| {
        b.iter()
            .filter(|&&x| x != 0.)
            .count()
            .cmp(&a.iter().filter(|&&x| x != 0.).count())
    });
    matrix
}

fn swap_with_nonzero_row(matrix: &mut Vec<Vec<f32>>, row_index: usize) {
    for i in (row_index + 1)..matrix.len() {
        if matrix[i][row_index] != 0. {
            matrix.swap(row_index, i);
            break;
        }
    }
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
