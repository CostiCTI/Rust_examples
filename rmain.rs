/*
Rust
The sum of the elements of a matrix

2
1 2    ->  10
3 4
*/

fn read_int() -> i64 {
    // Read integer 
    let cin = std::io::stdin();
    let mut input = String::new();
    cin.read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse::<i64>().unwrap();
    n
}

fn read_int_line() -> Vec<i64> {
    // Read vector of integers
    let cin = std::io::stdin();
    let mut line = String::new();
    cin.read_line(&mut line).unwrap();
    let values: Vec<i64> = line
                            .split_whitespace()
                            .map(|x| x.parse::<i64>().unwrap())
                            .collect();
    values
}

fn matrix_sum(mat: &Vec<Vec<i64>>) -> i64 {
    // Get matrix elements sum
    let mut sum: i64 = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            sum += mat[i][j];
        }
    }
    sum
}

fn main() {
    let n: i64 = read_int();
    let mut a: Vec<Vec<i64>> = Vec::new();
    for _ in 0..n {
        let values: Vec<i64> = read_int_line();
        a.push(values);
    }
    let sum: i64 = matrix_sum(&a);

    println!("{}", sum);
}