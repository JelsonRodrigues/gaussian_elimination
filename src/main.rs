use std::{ops::IndexMut, thread::JoinHandle};

use array2d::Array2D;
use linearsystem::LinearSystem;
use rand::{thread_rng, Rng};
pub mod array2d;
pub mod linearsystem;

fn main() {    
    // Ordem da matriz
    let n = 2;

    // Eliminação gaussiana é para resolver sistema onde
    // A * X = B
    // Onde A é uma matriz de ordem N
    // X é um vetor coluna de N posições
    // B é o resultado da multiplicação, um vetor de N valores

    // O código original utiliza alocação estática para A, B e X.
    // Eu utilizarei inicialmente alocação dinâmica no heap

    let (mut A, mut B, mut X) = create_values(n);


    // Teste rápido
    /*
    Sistema:
    2x + 3y = 10
    1x + 4y = 10
    
    A
    |2, 3|
    |1, 4|

    B = |10, 10|
    
    Resultado esperado de A*X=B
    X = |2, 2| 
     */

    A[0][0] = 2.0;
    A[0][1] = 3.0;
    A[1][0] = 1.0;
    A[1][1] = 4.0;

    B[0] = 10.0;
    B[1] = 10.0;

    for linha in &A {
        println!("{:?}", linha);
    }
    println!("B: {:?}", B);
    println!("X: {:?}", X);

    gauss_solver(&mut A, &mut B, &mut X);

    
    for linha in &A {
        println!("{:?}", linha);
    }
    println!("B: {:?}", B);
    println!("X: {:?}", X);

}

// Create the A, B and X values
fn create_values(n:usize) -> (Array2D<f64>, Vec<f64>, Vec<f64>) {
    let mut A:Array2D<f64> = Array2D::new(n, n);
    let mut B:Vec<f64> = vec![0.0;n];
    let mut X:Vec<f64> = vec![0.0;n];

    let mut random = thread_rng();

    for row in 0..A.rows_len() {
        random.try_fill(A.index_mut(row)).expect("Erro ao preencher A");
    }
    random.try_fill(&mut B[..]).expect("Erro ao preencher vetor B");
    return (A, B, X);
}


fn gauss_solver(A:&mut Array2D<f64>, B:&mut Vec<f64>, X:&mut Vec<f64>) {

    let N = A.columns_len();
    
    for norm in 0..(N-1) {
        for row in (norm + 1)..N {
            let multiplier = A[row][norm] / A[norm][norm];
            for col in norm..N {
                A[row][col] -= A[norm][col] * multiplier;
            }
            B[row] -= B[norm] * multiplier;
        }
    }

    for row in (0..N).rev() {
        X[row] = B[row];
        for col in ((row+1)..N).rev() {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }
}

fn gauss_multithread(A:&mut Array2D<f64>, B:&mut Vec<f64>, X:&mut Vec<f64>) {
    let threads: Vec<JoinHandle<(usize, f64)>> = Vec::new();

    for index in 0..A.columns_len() {
        let base = &A[index];
        for row in (index + 1)..A.columns_len() {
            let row_change = &mut A[row];
            let therad = std::thread::spawn(move || {
                row_solve(base, index, row_change, B[index],&mut B[row])
            });

        }
    } 
}

fn row_solve(base_row:&[f64], index_column:usize, row_changing:&mut [f64], base_res:f64, changing_res:&mut f64) -> (usize, f64) {
    let multiplier = row_changing[index_column] / base_row[index_column];

    for ind in index_column..row_changing.len(){
        row_changing[ind] -= base_row[ind] * multiplier;
    }
    *changing_res -= base_res * multiplier;
    return (index_column, multiplier);
}