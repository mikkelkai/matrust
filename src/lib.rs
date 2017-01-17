extern crate rand;
extern crate num;

use rand::Rng;
use std::fmt;
use num::Num;

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    val: Vec<T>,
}

impl<T: Clone + Num + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for i in 0..self.rows {
            for j in 0..self.cols {
                try!(write!(f, "{}", self.val[i * self.cols + j]));
                if j < self.cols - 1 {
                    try!(write!(f, ", "));
                }
            }
            if i < self.rows - 1 {
                try!(write!(f, "; "));
            }
        }
        write!(f, "]")
    }
}

impl<T: Clone + Num + std::iter::Sum> Matrix<T> {
    pub fn new(m: usize, n: usize, value: T) -> Matrix<T> {
        Matrix {
            rows: m,
            cols: n,
            val: vec![value; m * n],
        }
    }

    fn with_val(m: usize, n: usize, val: Vec<T>) -> Matrix<T>{
        Matrix {
            rows: m,
            cols: n,
            val: val,
        }
    }

    pub fn apply_fn<F>(&self, f: F) -> Matrix<T>
        where F: Fn(T) -> T {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            val: self.val.clone().into_iter().map(|x| f(x)).collect()
        }
    }

    pub fn scale(&self, scalar: T) -> Matrix<T> {
        self.apply_fn((|x| x * scalar.clone()))
    }

    pub fn mult_vec(&self, v: &Vec<T>) -> Result<Vec<T>, &str> {
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

    pub fn transpose(&self) -> Matrix<T> {
        let mut val = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.cols {
            for j in 0..self.rows {
                val.push(self.val[j * self.cols + i].clone())
            }
        }

        Matrix::with_val(self.cols, self.rows, val)
    }

    pub fn index(&self, m: usize, n: usize) -> Result<T, &str> {
        if m > self.rows || n > self.cols {
            return Err("Index out of range")
        }
        Ok(self.val[m * self.cols + n].clone())
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn mult_apply_fn<F>(&self, other: &Matrix<T>, f: F) -> Result<Matrix<T>, &str> 
    where F: Fn(T, T) -> T{
        if self.dimensions() != other.dimensions() {
            return Err("Matricies must be same size")
        }
        Ok(Matrix::with_val(self.rows, self.cols, (0..self.val.len()).map(|i| f(self.val[i].clone(), other.val[i].clone())).collect()))
    }

    pub fn add(&self, other: &Matrix<T>) -> Result<Matrix<T>, &str> {
        self.mult_apply_fn(other, (|x, y| x + y))
    }

    pub fn subtract(&self, other: &Matrix<T>) -> Result<Matrix<T>, &str> {
        self.mult_apply_fn(other, (|x, y| x - y))
    }

    pub fn push(&mut self, m: usize, n: usize, val: T) -> Result<(), &str> {
        if m > self.rows || n > self.cols {
            return Err("Index out of range")
        }
        self.val[m*self.cols+n] = val;
        Ok(())
    }
}

impl<T: Clone + Num + std::iter::Sum + rand::Rand> Matrix<T> {
    pub fn rand(m: usize, n: usize) -> Matrix<T> {
        let mut rng = rand::thread_rng();
        Matrix::with_val(m, n, (0..m * n).map(|_| rng.gen()).collect())
    }
}

impl<T: Clone + Num> PartialEq for Matrix<T> {
    fn eq (&self, other: &Matrix<T>) -> bool {
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
    let v = m.mult_vec(&vec![2.0, 1.0]);
    assert_eq!(v, Ok(vec![4.0, 10.0]));
}

#[test]
fn test_transpose() {
    let m = Matrix::with_val(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(m.transpose(), Matrix::with_val(2, 2, vec![1.0, 3.0, 2.0, 4.0]));
    let m = Matrix::with_val(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    assert_eq!(m.transpose(), Matrix::with_val(3, 3, vec![1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0]));
    let m = Matrix::with_val(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(m.transpose(), Matrix::with_val(2, 3, vec![1.0, 3.0, 5.0, 2.0, 4.0, 6.0]));
}

#[test]
fn test_index() {
    let m = Matrix::with_val(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(m.index(0, 0), Ok(1.0));
    assert_eq!(m.index(0, 1), Ok(2.0));
    assert_eq!(m.index(1, 0), Ok(3.0));
    assert_eq!(m.index(1, 1), Ok(4.0));
}

#[test]
fn test_dimensions() {
    assert_eq!(Matrix::new(1, 1, 1.0).dimensions(), (1, 1));
    assert_eq!(Matrix::new(1, 2, 1.0).dimensions(), (1, 2));
    assert_eq!(Matrix::new(2, 1, 1.0).dimensions(), (2, 1));
    assert_eq!(Matrix::new(2, 2, 1.0).dimensions(), (2, 2));
}

#[test]
fn test_add() {
    let m1 = Matrix::with_val(2, 3, vec![2.0, 4.0, 5.0, 1.0, 2.0, 3.0]);
    let m2 = Matrix::with_val(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(m1.add(&m2), Ok(Matrix::with_val(2, 3, vec![3.0, 6.0, 8.0, 5.0, 7.0, 9.0])));
}

#[test]
fn test_subtraction() {
    let m1 = Matrix::with_val(2, 3, vec![2.0, 4.0, 5.0, 1.0, 2.0, 3.0]);
    let m2 = Matrix::with_val(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(m1.subtract(&m2), Ok(Matrix::with_val(2, 3, vec![1.0, 2.0, 2.0, -3.0, -3.0, -3.0])));
}

#[test]
fn test_push() {
    let mut m = Matrix::new(3,3, 1.0);
    m.push(0, 2, 2.0);
    assert_eq!(m.index(0, 2), Ok(2.0));
    m.push(2, 2, 2.0);
    assert_eq!(m.index(2, 2), Ok(2.0));
}

#[test]
fn test_display() {
    let m: Matrix<f64> = Matrix::rand(2, 3);
    println!("{}", m);
}