use nalgebra::{DMatrix, DVector, Scalar};
use polynomen::Poly;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn get_a_t(m: usize, n: usize, a: &Vec<f64>) -> DMatrix<f64> {
    let A_t = DMatrix::from_fn(m, n, |i, j| a[j].powi(i as i32));
    println!("A_t: {}", A_t);
    A_t
}

fn get_g(n: usize, k: usize, a: &Vec<f64>) -> DMatrix<f64> {
    let G = DMatrix::from_fn(n, k, |i, j| a[i].powi(j as i32));
    println!("G: {}", G);
    G
}

fn get_b_t(n: usize, a: &Vec<f64>) -> DMatrix<f64> {
    let mut B_t: DMatrix<f64> = DMatrix::zeros(n, n);
    for row in (0..n) {
        let M = Poly::new_from_roots_iter(
            a.iter()
                .enumerate()
                .filter(|&(i, _)| i != row)
                .map(|(_, &v)| v),
        );
        let coeff = M.coeffs();
        let N_i: f64 = a
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != row)
            .map(|(_, &v)| a[row] - v)
            .product();
        println!("M: {}", M);
        println!("N_i: {}", N_i);
        for col in (0..n) {
            B_t[(row, col)] = coeff[col] / N_i;
        }
    }
    println!("B_t: {}", B_t);
    B_t
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let kernel: DMatrix<f64> =
        parse::<f64, _>(BufReader::new(File::open(args[2].clone()).unwrap())).unwrap();
    println!("kernel: {}", kernel);

    let output_dimension: usize = args[3].parse::<usize>().unwrap();
    println!("output_dimension: {}", output_dimension);

    let input_size = kernel.nrows() + output_dimension - 1;

    let input: DMatrix<f64> =
        parse::<f64, _>(BufReader::new(File::open(args[1].clone()).unwrap())).unwrap();
    assert!(
        input_size == input.nrows(),
        "Input size does not meet output and kernel constraint."
    );
    println!("input: {}", input);

    let interp_points: Vec<f64> = (0..input_size).map(|x| x as f64).collect();
    println!("interp_points: {:?}", interp_points);

    let m = output_dimension;
    let n = input_size;
    let k = kernel.nrows();

    let A_t = get_a_t(m, n, &interp_points);
    let G = get_g(n, k, &interp_points);
    let B_t = get_b_t(n, &interp_points);

    let filter_transform: DMatrix<f64> = G.clone() * kernel * G.transpose();
    println!("filter_transform: {}", filter_transform);

    let input_transform: DMatrix<f64> = B_t.clone() * input * B_t.transpose();
    println!("input_transform: {}", input_transform);

    let mut core: DMatrix<f64> = filter_transform.component_mul(&input_transform);
    println!("core: {}", core);

    let output: DMatrix<f64> = A_t.clone() * (core) * A_t.transpose(); // of size dimension
    println!("output: {}", output);
}

/// Courtesy https://github.com/dimforge/nalgebra/issues/325
/// Consumes a `BufRead` of line and comma delimited numbers, and
/// produces either a `DMatrix` or an error.
fn parse<N, R>(input: R) -> Result<DMatrix<N>, Box<dyn Error>>
where
    N: FromStr + Scalar,
    N::Err: Error,
    R: BufRead,
{
    // initialize an empty vector to fill with numbers
    let mut data = Vec::new();

    // initialize the number of rows to zero; we'll increment this
    // every time we encounter a newline in the input
    let mut rows = 0;

    // for each line in the input,
    for line in input.lines() {
        // increment the number of rows
        rows += 1;
        // iterate over the items in the row, separated by commas
        for datum in line?.split_terminator(",") {
            // trim the whitespace from the item, parse it, and push it to
            // the data array
            data.push(N::from_str(datum.trim())?);
        }
    }

    // The number of items divided by the number of rows equals the
    // number of columns.
    let cols = data.len() / rows;

    // Construct a `DMatrix` from the data in the vector.
    Ok(DMatrix::from_row_slice(rows, cols, &data[..]))
}
