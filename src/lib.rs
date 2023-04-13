pub mod multrix {
    use rand::Rng;
    use rayon::prelude::*;
    use std::fs;

    /// A matrix struct that stores the values in a one-dimensional vector, where each row is stored
    /// contiguously.
    pub struct Matrix {
        data: Vec<f64>,
        rows: usize,
        cols: usize,
    }

    impl Matrix {
        /// Creates a new (square) identity matrix with the given dimension.
        pub fn new_identity(dimension: usize) -> Matrix {
            let mut data: Vec<f64> = vec![0.0; dimension * dimension];

            for i in 0..dimension {
                for j in 0..dimension {
                    data[i * dimension + j] = {
                        if i == j {
                            1.0
                        } else {
                            0.0
                        }
                    };
                }
            }
            Matrix { data, rows: dimension, cols: dimension }
        }

        /// Creates a new matrix with the given dimensions and random values.
        pub fn new_rand(rows: usize, cols: usize) -> Matrix {
            let mut data: Vec<f64> = vec![0.0; cols * rows];

            let mut rng = rand::thread_rng();
            for i in 0..rows {
                for j in 0..cols {
                    data[i * cols + j] = rng.gen_range(0..10) as f64;
                }
            }
            Matrix { data, rows, cols }
        }

        /// Creates a new matrix with the given dimensions reading the values from a file.
        /// The file must contain a comma-separated list of numbers, with each row on a new line.
        /// The last element on each row may or may not be followed by a comma.
        ///
        /// # Panics
        /// The function panics if the dimensions are incorrect, if it fails to read from the file,
        /// or the file contains invalid data and numbers cannot be parsed.
        pub fn new_from_file(filename: &str) -> Matrix {
            let contents = match fs::read_to_string(filename) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "{} failed to read from file '{}': {:?}",
                        "Error:", filename, e
                    );
                    panic!();
                }
            };
            let mut data = Vec::new();
            let rows = contents.lines().count();
            let mut cols = 0;
            for line in contents.lines() {
                for num_str in line.split(',') {
                    data.push(num_str.parse().unwrap());
                }
                if cols == 0 {
                    cols = line.split(',').count();
                }
            }
            Matrix { data, rows, cols }
        }

        /// Creates a new matrix with the given dimensions from a one-dimensional vector containing
        /// the values, where each row is stored contiguously.
        ///
        /// # Panics
        /// The function panics if the provided dimensions are different than the vector length.
        pub fn new_from_vec(data: Vec<f64>, rows: usize, cols: usize) -> Matrix {
            assert_eq!(data.len(), rows * cols, "Invalid matrix dimensions");
            Matrix { data, rows, cols }
        }

        /// Creates a new matrix with the given dimensions from a two-dimensional vector containing
        /// the values.
        pub fn new_from_vec_vec(data: Vec<Vec<f64>>) -> Matrix {
            let rows = data.len();
            let cols = data[0].len();
            let mut data_vec = Vec::with_capacity(rows * cols);
            for row in data {
                for element in row {
                    data_vec.push(element);
                }
            }
            Matrix { data: data_vec, rows, cols }
        }

        /// Gets the value at the given row and column indices.
        pub fn get(&self, row: usize, col: usize) -> f64 {
            self.data[row * self.cols + col]
        }

        /// Sets the value at the given row and column indices.
        pub fn set(&mut self, row: usize, col: usize, value: f64) {
            self.data[row * self.cols + col] = value;
        }

        /// Returns the current matrix transposed (rows and columns swapped).
        pub fn transpose(&self) -> Matrix {
            let mut data = vec![0.0; self.rows * self.cols];
            for i in 0..self.rows {
                for j in 0..self.cols {
                    data[j * self.rows + i] = self.data[i * self.cols + j];
                }
            }
            Matrix { data, rows: self.cols, cols: self.rows }
        }

        /// Returns whether the two matrices are conformable for multiplication.
        pub fn is_conformable(&self, other: &Matrix) -> bool {
            self.cols == other.rows
        }

        /// Adds the given matrix to the current one and returns the result.
        ///
        /// # Panics
        /// The function panics if the matrices cannot be added: they must have the same dimensions.
        fn addition(self, other: Matrix) -> Matrix {
            assert_eq!(self.rows, other.rows, "Matrices cannot be added");
            assert_eq!(self.cols, other.cols, "Matrices cannot be added");
            let mut data = vec![0.0; self.rows * self.cols];
            for i in 0..self.rows * self.cols {
                data[i] = self.data[i] + other.data[i];
            }
            Matrix { data, rows: self.rows, cols: self.cols }
        }

        /// Negates the sign of the current matrix and returns the result.
        fn negation(self) -> Matrix {
            let mut data = vec![0.0; self.rows * self.cols];
            for i in 0..self.rows * self.cols {
                data[i] = -self.data[i];
            }
            Matrix { data, rows: self.rows, cols: self.cols }
        }

        /// Returns the product between the current matrix and the given one, and uses only one thread.
        ///
        /// # Panics
        /// The function panics if the matrices cannot be multiplied: the number of columns of the
        /// first matrix must be equal to the number of rows of the second matrix.
        pub fn product(self, other: Matrix) -> Matrix {
            if self.cols != other.rows {
                panic!("Matrices cannot be multiplied");
            }

            let rows = self.rows;
            let cols = other.cols;
            let mut data = vec![0.0; cols * rows];
            for i in 0..cols * rows {
                let mut c = 0.0;
                let row = i / cols;
                let col = i % cols;
                for k in 0..self.cols {
                    c += self.data[row * self.cols + k] * other.data[k * other.cols + col];
                }
                data[i] = c;
            }
            Matrix { data, rows, cols }
        }

        /// Returns the product between the current matrix and the given one, and uses multiple threads.
        ///
        /// # Panics
        /// The function panics if the matrices cannot be multiplied: the number of columns of the
        /// first matrix must be equal to the number of rows of the second matrix.
        pub fn parallel_product(self, other: Matrix) -> Matrix {
            if self.cols != other.rows {
                panic!("Matrices cannot be multiplied");
            }

            let rows = self.rows;
            let cols = other.cols;
            let mut data = vec![0.0; cols * rows];

            data.par_iter_mut().enumerate().for_each(|(i, c)| {
                let row = i / cols;
                let col = i % cols;
                for k in 0..self.cols {
                    *c += self.data[row * self.cols + k] * other.data[k * other.cols + col];
                }
            });

            Matrix { data, rows, cols }
        }

        /// Writes the matrix to the given file in the same comma-separated format as the input.
        ///
        /// # Panics
        /// The function panics if it fails to write to the file.
        pub fn write_to_file(&self, filename: &str) {
            let mut matrix_str = String::with_capacity(self.rows * self.cols * 2);
            for (i, element) in self.data.iter().enumerate() {
                matrix_str.push_str(&format!("{},", element));
                if (i + 1) % self.cols == 0 {
                    matrix_str.push('\n');
                }
            }

            match fs::write(filename, matrix_str) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: failed to write to file '{}': {:?}", filename, e);
                    panic!();
                }
            }
        }

    }
    use std::ops::Add;
    impl Add for Matrix {
        type Output = Matrix;
        fn add(self, other: Matrix) -> Matrix {
            self.addition(other)
        }
    }

    use std::ops::Neg;
    impl Neg for Matrix {
        type Output = Matrix;
        fn neg(self) -> Matrix {
            self.negation()
        }
    }

    use std::ops::Mul;
    impl Mul for Matrix {
        type Output = Matrix;
        fn mul(self, other: Matrix) -> Matrix {
            self.parallel_product(other)
        }
    }

    use std::ops::Sub;
    impl Sub for Matrix {
        type Output = Matrix;
        fn sub(self, other: Matrix) -> Matrix {
            self + (-other)
        }
    }

    use std::fmt;
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (i, element) in self.data.iter().enumerate() {
                write!(f, "{},", element)?;
                if (i + 1) % self.cols == 0 {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }
}
