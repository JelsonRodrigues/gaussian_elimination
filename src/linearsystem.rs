use crate::array2d::Array2D;

pub struct LinearSystem {
    A:Array2D<f64>, // The coeficients for the system
    X:Vec<f64>,     // The incognitas
    B:Vec<f64>      // The equal part
}

impl LinearSystem {
    pub fn new(A:Array2D<f64>, B:Vec<f64>) -> Self {
        let order = A.columns_len();
        LinearSystem { A: A, X: vec![0.0; order], B: B }
    }

    pub fn solve_with_gauss(&mut self) {
        let N = self.A.columns_len();
        
        for norm in 0..(N-1) {
            for row in (norm + 1)..N {
                let multiplier = self.A[row][norm] / self.A[norm][norm];
                for col in norm..N {
                    self.A[row][col] -= self.A[norm][col] * multiplier;
                }
                self.B[row] -= self.B[norm] * multiplier;
            }
        }

        for row in (0..N).rev() {
            self.X[row] = self.B[row];
            for col in ((row+1)..N).rev() {
                self.X[row] -= self.A[row][col] * self.X[col];
            }
            self.X[row] /= self.A[row][row];
        }
    }

    pub fn print(&self){
        for index_line in 0..self.A.rows_len() {
            print!("| ");
            for valor in &self.A[index_line] {
                print!("{valor:.2}\t");
            }
            print!("| * |{:.2}| = |{:.2}|\n", self.X[index_line], self.B[index_line]);
        }
    }
}