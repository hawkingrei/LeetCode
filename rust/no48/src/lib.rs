pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len() - 1;
    for x in 0..matrix.len() / 2 {
        for y in 0..(matrix.len() + 1) / 2 {
            let tmp = matrix[x][y];
            matrix[x][y] = matrix[n - y][x];
            matrix[n - y][x] = matrix[n - x][n - y];
            matrix[n - x][n - y] = matrix[y][n - x];
            matrix[y][n - x] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
