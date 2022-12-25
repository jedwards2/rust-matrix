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
    let short_row: i32;
    let long_row: i32;

    if matrix.m < matrix.n {
        short_row = matrix.m;
        long_row = matrix.n;
    } else {
        short_row = matrix.n;
        long_row = matrix.m;
    }

    let mut test_matrix = Matrix {
        array: matrix.array,
        m: matrix.m,
        n: matrix.n,
    };

    for i in 0..(matrix.m) {
        for q in 0..(matrix.n) {
            test_matrix.array[((i * short_row) + q) as usize] =
                matrix.array[((q * long_row) + i) as usize];
        }
    }

    matrix.array = test_matrix.array;

    return matrix;
}
