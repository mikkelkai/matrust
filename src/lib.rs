extern crate rand;

use rand::Rng;

#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    val: Vec<f64>,
}

impl Matrix {
    pub fn new(m: usize, n: usize, value: f64) -> Matrix {
        Matrix {
            rows: m,
            cols: n,
            val: vec![value; m * n],
        }
    }

    fn with_val(m: usize, n: usize, val: Vec<f64>) -> Matrix{
        Matrix {
            rows: m,
            cols: n,
            val: val,
        }
    }

    pub fn rand(m: usize, n: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        Matrix::with_val(m, n, (0..m * n).map(|_| rng.gen()).collect())
    }

    pub fn apply_fn<F>(&self, f: F) -> Matrix
        where F: Fn(f64) -> f64 {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            val: self.val.clone().into_iter().map(|x| f(x)).collect()
        }
    }

    pub fn scale(&self, scalar: f64) -> Matrix {
        self.apply_fn((|x| x * scalar))
    }

    pub fn mult_vec(&self, v: Vec<f64>) -> Result<Vec<f64>, &str> {
        if v.len() != self.cols {
            return Err("Vector length must equal matrix column length")
        }
        Ok((0..self.rows)
            .map(|i| {
                (0..self.cols)
                    .map(|j| v[j].clone() * self.val[i * self.cols + j].clone())
                    .sum()
            })
            .collect())
    }
}

impl PartialEq for Matrix {
    fn eq (&self, other: &Matrix) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.val == other.val
    }
}

#[test]
fn test_new() {
    let m = Matrix::new(2, 2, 1.0);
    assert_eq!(m, Matrix{rows: 2, cols: 2, val: vec![1.0, 1.0, 1.0, 1.0]});
}

#[test]
fn test_rand() {
    let m = Matrix::rand(2, 2);
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 2);
    assert!(0.0 <= m.val[0] && m.val[0] <= 1.0);
    assert!(0.0 <= m.val[0] && m.val[1] <= 1.0);
    assert!(0.0 <= m.val[0] && m.val[2] <= 1.0);
    assert!(0.0 <= m.val[0] && m.val[3] <= 1.0);
}

#[test]
fn test_with_val() {
    let m = Matrix::with_val(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(m, Matrix{rows: 2, cols: 2, val: vec![1.0, 2.0, 3.0, 4.0]});
}

#[test]
fn test_apply_fn() {
    let m = Matrix::with_val(2, 2, vec![1.0, 2.0, 3.0, 4.0]).apply_fn((|x| x*x));
    assert_eq!(m, Matrix{rows: 2, cols: 2, val: vec![1.0, 4.0, 9.0, 16.0]});
}

#[test]
fn test_scale() {
    let m = Matrix::with_val(2, 2, vec![1.0, 2.0, 3.0, 4.0]).scale(2.0);
    assert_eq!(m, Matrix{rows: 2, cols: 2, val: vec![2.0, 4.0, 6.0, 8.0]});
}

#[test]
fn test_mult_vec() {
    let m = Matrix::with_val(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let v = m.mult_vec(vec![2.0, 1.0]);
    assert_eq!(v, Ok(vec![4.0, 10.0]));
}