struct Matrix {
    array: [i32; 9],
    m: i32,
    n: i32,
}

fn main() {
    let matrix = Matrix {
        array: [0, 1, 2, 3, 4, 5, 6, 7, 8],
        m: 3,
        n: 3,
    };
    display_matrix(&matrix);
}

fn display_matrix(matrix: &Matrix) {
    for i in 0..(matrix.m) {
        for q in 0..(matrix.n) {
            print!("{}", matrix.array[((i * 3) + q) as usize])
        }
        println!();
    }
}
