use rand::{thread_rng, Rng};
use std::{thread, thread::JoinHandle};
pub mod array2d;
pub mod linearsystem;

fn main() {
    // Ordem da matriz
    let n = 5000;

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

    println!("------- Normal -------");
    println!("------- Antes -------");
    A[0][0] = 2.0;
    A[0][1] = 3.0;
    A[1][0] = 1.0;
    A[1][1] = 4.0;

    B[0] = 10.0;
    B[1] = 10.0;

    X[0] = 0.0;
    X[1] = 0.0;

    gauss_solver_with_unsafe(&mut A, &mut B, &mut X);

    // for linha in &A {
    //     println!("{:?}", linha);
    // }
    // println!("B: {:?}", B);
    println!("X: {:?}", X);
}

// Create the A, B and X values
fn create_values(n: usize) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let mut random = thread_rng();
    let mut A: Vec<Vec<f64>> = Vec::new();
    let mut B: Vec<f64> = vec![0.0; n];
    let mut X: Vec<f64> = vec![0.0; n];

    for row in 0..n {
        let mut temp = vec![0.0; n];
        random.try_fill(&mut temp[..]);
        A.push(temp);
    }

    random.try_fill(&mut B[..]);
    random.try_fill(&mut X[..]);
    return (A, B, X);
}

/*
This function is based on the original https://github.com/gmendonca/gaussian-elimination-pthreads-openmp/blob/master/gauss.c
 */
fn gauss_solver(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    let N = A.len();

    for norm in 0..(N - 1) {
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
        for col in ((row + 1)..N).rev() {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }
}

fn row_solver(row_normalizing: &mut Vec<f64>, base_row: &Vec<f64>, index: usize) {
    let multiplier = row_normalizing[index] / base_row[index];

    for ind_col in index..row_normalizing.len() {
        row_normalizing[ind_col as usize] -= multiplier * base_row[ind_col as usize];
    }
}

fn row_solver_with_simd(row_normalizing: &mut Vec<f64>, base_row: &Vec<f64>, index: usize) {
    todo!()
}

/*
TODO:
    Create a threadpool and in each iteration of the loop add the tasks to the threadpool
    and wait for finish execution of current tasks in threadpool before moving to next iteration
 */
fn gauss_solver_with_unsafe(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    let n = A.len();

    for norm_row in 0..n {
        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        let base_row = unsafe { &*A.as_ptr().offset(norm_row as isize) };

        for ind_row in (norm_row + 1)..n {
            // This part can be done in a Thread
            let row_normalizing = unsafe { &mut *A.as_mut_ptr().offset(ind_row as isize) };
            let multiplier = row_normalizing[norm_row] / base_row[norm_row];

            let thread = thread::spawn(move || {
                row_solver(row_normalizing, base_row, norm_row);
            });
            threads.push(thread);

            B[ind_row] -= multiplier * B[norm_row];
        }

        // wait for threads to finish
        for thread in threads {
            thread.join().unwrap();
        }
    }

    for row in (0..n).rev() {
        X[row] = B[row];
        for col in ((row + 1)..n).rev() {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }
}
