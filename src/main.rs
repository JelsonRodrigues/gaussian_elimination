use rand::{thread_rng, Rng, distributions::uniform::SampleBorrow, rngs::StdRng, SeedableRng};
use std::{thread, thread::JoinHandle, time::{Instant, Duration}};

fn main() {
    // Ordem da matriz
    let n = 7;

    // Eliminação gaussiana é para resolver sistema onde
    // A * X = B
    // Onde A é uma matriz de ordem N
    // X é um vetor coluna de N posições
    // B é o resultado da multiplicação, um vetor de N valores

    let (mut A, mut B, mut X) = create_values(n);


    let before = Instant::now();
    // gauss_solver_with_unsafe(&mut A, &mut B, &mut X);
    // gauss_solver(&mut A, &mut B, &mut X);
    gauss_solver_with_futures(&mut A, &mut B, &mut X);
    let now = Instant::now();

    println!("X: {:?}", X);
    show_time((now - before).borrow());
}

 fn show_time(duration: &Duration) {
    let ms = duration.as_millis() % 1000;
    let s = duration.as_secs() % 60;
    let m = duration.as_secs() / 60 % 60;
    let h = duration.as_secs() / 60 / 60;
    println!("Total time: {}s", duration.as_secs_f64());
    println!("\t{h:02}h:{m:02}m:{s:02}s:{ms:03}ms"); 
}

// Create the A, B and X values
fn create_values(n: usize) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    // let mut random = thread_rng();
    let mut r = StdRng::seed_from_u64(0);   // Reproducible random sequence
    let mut A: Vec<Vec<f64>> = Vec::new();
    let B: Vec<f64> = vec![1.0; n];
    let X: Vec<f64> = vec![0.0; n];

    for _ in 0..n {
        let mut temp = vec![0.0; n];
        r.try_fill(&mut temp[..]).unwrap();
        A.push(temp);
    }

    // random.try_fill(&mut B[..]).unwrap();
    // random.try_fill(&mut X[..]).unwrap();
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

async fn gauss_solver_with_futures(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    // Tokio runtime starting
    let tk = tokio::runtime::Builder::new_multi_thread()
                            .build()
                            .unwrap();
    let n = A.len();

    for norm_row in 0..n {
        let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

        let base_row = unsafe { &*A.as_ptr().offset(norm_row as isize) };

        for ind_row in (norm_row + 1)..n {
            // This part can be done in a Thread
            let row_normalizing = unsafe { &mut *A.as_mut_ptr().offset(ind_row as isize) };
            let multiplier = row_normalizing[norm_row] / base_row[norm_row];

            let task = tk.spawn(async move {row_solver(row_normalizing, base_row, norm_row)});
            
            // let thread = thread::spawn(move || {
            //     row_solver(row_normalizing, base_row, norm_row);
            // });
            // threads.push(thread);
            tasks.push(task);
            B[ind_row] -= multiplier * B[norm_row];
        }

        // wait for threads to finish
        for task in tasks {
            // task.await.unwrap();
            tk.block_on(task).unwrap();
        }
    }
    tk.shutdown_background();

    for row in (0..n).rev() {
        X[row] = B[row];
        for col in ((row + 1)..n).rev() {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }
}