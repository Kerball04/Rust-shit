use ndarray::{array, Array2};

fn matrix_multiply(matrix1: Array2<i32>, matrix2: Array2<i32>) -> Array2<i32> {
    let result = matrix1.dot(&matrix2);

    result
}

fn main() {
    let matrix1 = array! [
        [1, 2, 3]
    ];

    let matrix2 = array! [
        [4],
        [5],
        [6]
    ];

    let result = matrix_multiply(matrix1, matrix2);
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let matrix1 = array! [
            [1, 2, 3], 
            [4, 5, 6], 
            [7, 8, 9]
        ];

        let matrix2 = array! [
            [10],
            [11],
            [12]
        ];

        let expected = array![
            [68],
            [167],
            [266]
        ];

        let result = matrix_multiply(matrix1, matrix2);

        assert_eq!(result, expected);
    }
}