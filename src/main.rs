struct Matrix {
    array: [i32; 9],
    m: i32,
    n: i32,
}

fn main() {
    let mut matrix = Matrix {
        array: [0, 1, 2, 3, 4, 5, 6, 7, 8],
        m: 3,
        n: 3,
    };

    display_matrix(&matrix);
    transpose_matrix(&mut matrix);
    display_matrix(&matrix);
}

fn display_matrix(matrix: &Matrix) {
    let short_row: i32;
    if matrix.m < matrix.n {
        short_row = matrix.m;
    } else {
        short_row = matrix.n
    }

    for i in 0..(matrix.m) {
        for q in 0..(matrix.n) {
            print!("{}", matrix.array[((i * short_row) + q) as usize])
        }
        println!();
    }
}

fn transpose_matrix(matrix: &mut Matrix) -> &mut Matrix {
    //implement
    return matrix;
}
