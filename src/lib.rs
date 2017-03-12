extern crate num;

use num::Num;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Matrix<A> {
    rows: usize,
    cols: usize,
    val: Vec<A>,
}

impl<A: Clone + fmt::Display> fmt::Display for Matrix<A> {
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

impl<A: Clone> Matrix<A> {
    // Initialization
    pub fn new(rows: usize, cols: usize, val: A) -> Matrix<A> {
        Matrix {
            rows: rows,
            cols: cols,
            val: vec![val; rows * cols],
        }
    }

    pub fn new_with_val(rows: usize, cols: usize, val: Vec<A>) -> Result<Matrix<A>, &'static str> {
        if val.len() != rows * cols {
            return Err("Value length needs to equal rows * cols");
        }
        Ok(Matrix {
            rows: rows,
            cols: cols,
            val: val,
        })
    }

    // Indexing
    pub fn index(&self, row: usize, col: usize) -> Result<A, &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("Index out of range");
        }
        Ok(self.val[row * self.cols + col].clone())
    }

    // Inserting
    pub fn insert(&mut self, row: usize, col: usize, value: A) -> Result<(), &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("Index out of range");
        }
        self.val[row * self.cols + col] = value;
        Ok(())
    }

    // Dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    // Mapping
    pub fn map<F, B>(&self, f: F) -> Matrix<B>
        where F: Fn(A) -> B
    {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            val: self.val.clone().into_iter().map(f).collect(),
        }
    }

    pub fn map2<F, B: Clone, C>(&self, m: &Matrix<B>, f: F) -> Result<Matrix<C>, &'static str>
        where F: Fn(A, B) -> C
    {
        if self.dimensions() != m.dimensions() {
            return Err("Both matricies need to have same dimensions");
        }
        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            val: (0..self.rows * self.cols)
                .map(|i| f(self.val[i].clone(), m.val[i].clone()))
                .collect(),
        })
    }

    pub fn transpose(&self) -> Matrix<A> {
        let mut val = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.cols {
            for j in 0..self.rows {
                val.push(self.val[j * self.cols + i].clone())
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            val: val,
        }
    }

    // Applying
    /*pub fn apply<F: Clone, B: Clone>(&self, f: Matrix<Box<F>>) -> Result<Matrix<B>, &'static str>
        where F: Fn(A) -> B
    {
        if self.dimensions() == f.dimensions() {
            return Err("Both matricies need to have same dimensions");
        }
        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            val: (0..self.rows * self.cols).map(|i| f.val[i](self.val[i].clone())).collect(),
        })
    }*/
}

impl<A: Clone + Num> Matrix<A> {
    pub fn add(&self, m: &Matrix<A>) -> Result<Matrix<A>, &'static str> {
        self.map2(m, (|x, y| x + y))
    }

    pub fn sub(&self, m: &Matrix<A>) -> Result<Matrix<A>, &'static str> {
        self.map2(m, (|x, y| x - y))
    }

    pub fn scale(&self, n: A) -> Matrix<A> {
        self.map(|x| x * n.clone())
    }
}

impl<A: Clone + Num + std::iter::Sum> Matrix<A> {
    pub fn vec_mult(&self, v: &Vec<A>) -> Result<Vec<A>, &'static str> {
        if v.len() != self.cols {
            return Err("Vector length must equal matrix column length");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let m = Matrix::new(2, 2, 0);
        let em = Matrix {
            rows: 2,
            cols: 2,
            val: vec![0, 0, 0, 0],
        };
        assert_eq!(m.val, em.val);

        let m2 = Matrix::new(3, 4, 1);
        let em2 = Matrix {
            rows: 3,
            cols: 4,
            val: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        };
        assert_eq!(m2, em2);
    }

    #[test]
    fn new_with_val() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let m = Matrix::new_with_val(2, 3, v.clone()).unwrap();
        assert_eq!(m.val, v);
        // let m = matrix![1, 2, 3; 4, 5, 6]; the dreams that never came true
        assert!(Matrix::new_with_val(3, 3, vec![1]).is_err());
    }

    #[test]
    fn index() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        assert_eq!(m.index(0, 0).unwrap(), 1);
        assert_eq!(m.index(0, 1).unwrap(), 2);
        assert_eq!(m.index(1, 0).unwrap(), 3);
        assert_eq!(m.index(1, 1).unwrap(), 4);
        assert!(m.index(2, 0).is_err());
        assert!(m.index(0, 2).is_err());
    }

    #[test]
    fn insert() {
        let mut m = Matrix::new(2, 2, 0);
        m.insert(0, 1, 42).unwrap();
        assert_eq!(m.index(0, 1).unwrap(), 42);
    }

    #[test]
    fn dimensions() {
        assert_eq!(Matrix::new(2, 2, 0).dimensions(), (2, 2));
        assert_eq!(Matrix::new(22, 43, 0).dimensions(), (22, 43));
    }

    #[test]
    fn map() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = m.map(|x| x * 2);
        let m3 = Matrix::new_with_val(2, 2, vec![2, 4, 6, 8]).unwrap();
        assert_eq!(m2, m3);
    }

    #[test]
    fn map2() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m3 = m.map2(&m2, |x, y| x * y).unwrap();
        let m4 = Matrix::new_with_val(2, 2, vec![1, 4, 9, 16]).unwrap();
        assert_eq!(m3, m4);

        let m5 = Matrix::new(3, 3, 0);
        assert!(m.map2(&m5, (|x, y| y + x)).is_err());
    }

    #[test]
    fn add() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m3 = m.add(&m2).unwrap();
        let m4 = Matrix::new_with_val(2, 2, vec![2, 4, 6, 8]).unwrap();
        assert_eq!(m3, m4);

        let m5 = Matrix::new(3, 3, 0);
        assert!(m.add(&m5).is_err());
    }

    #[test]
    fn sub() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m3 = m.sub(&m2).unwrap();
        let m4 = Matrix::new(2, 2, 0);
        assert_eq!(m3, m4);

        let m5 = Matrix::new(3, 3, 0);
        assert!(m.sub(&m5).is_err());
    }

    #[test]
    fn scale() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = m.scale(2);
        let m3 = Matrix::new_with_val(2, 2, vec![2, 4, 6, 8]).unwrap();
        assert_eq!(m2, m3);
    }

    #[test]
    fn vec_mult() {
        let m = Matrix::new(3, 4, 3);
        let v = vec![1, 2, 3, 4];
        let v2 = m.vec_mult(&v).unwrap();
        let v3 = vec![30, 30, 30];
        assert_eq!(v2, v3);
    }

    #[test]
    fn transpose() {
        let m = Matrix::new_with_val(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m2 = m.transpose();
        assert_eq!(m2.dimensions(), (3, 2));
        let m3 = Matrix::new_with_val(3, 2, vec![1, 4, 2, 5, 3, 6]).unwrap();
        assert_eq!(m2, m3);
    }

    /* Closures pls
    #[test]
    fn apply() {
        let m = Matrix::new_with_val(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = Matrix::new_with_val(2,
                                      2,
                                      vec![Box::new((|x| x * 4)),
                                           Box::new((|x| x / 2)),
                                           Box::new((|x| x * 2)),
                                           Box::new((|x| x / 2))])
            .unwrap();
        let m3 = m.apply(m2).unwrap();
        let m4 = Matrix::new_with_val(2, 2, [4, 1, 6, 2]);
        assert_eq!(m3, m4);
    }*/

}