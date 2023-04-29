use rand::{thread_rng, Rng, distributions::uniform::SampleBorrow, rngs::StdRng, SeedableRng};
use std::{thread, thread::JoinHandle, time::{Instant, Duration}, ops::Range};

fn main() {
    // Ordem da matriz
    let n = 3000;

    let (mut A, mut B, mut X) = create_values(n);
    // let (mut A, mut B, mut X) = create_values_array2d(n);

    let before = Instant::now();
    // gauss_solver_with_thread_pool_chunks(&mut A, &mut B, &mut X);
    // gauss_solver_with_threads(&mut A, &mut B, &mut X);
    gauss_solver(&mut A, &mut B, &mut X);
    // gauss_solver_Array2D(&mut A, &mut B, &mut X);
    // gauss_solver_with_futures(&mut A, &mut B, &mut X);
    // gauss_solver_with_thread_pool(&mut A, &mut B, &mut X);
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

pub mod array2d;
use array2d::Array2D;
fn create_values_array2d(n: usize) -> (Array2D<f64>, Vec<f64>, Vec<f64>) {
    // let mut random = thread_rng();
    let mut r = StdRng::seed_from_u64(0);   // Reproducible random sequence
    let mut A = Array2D::new(n, n);
    let B: Vec<f64> = vec![1.0; n];
    let X: Vec<f64> = vec![0.0; n];

    for index in 0..n {
        r.try_fill(&mut A[index]).unwrap();
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

fn gauss_solver_Array2D(A: &mut Array2D<f64>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    let N = A.columns_len();

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

// Spawn a thread for each row of the matrix, this is EXTREMALLY bad and slow
// The overhead of thread creation and scheduling will be greater than
// any benefits, given that the work each thread has to do is small.
fn gauss_solver_with_threads(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
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

// Spawn a future (tokio green thread) for every row. 
// Futures overhead are small, so the performance is great.
use tokio::*;
fn gauss_solver_with_futures(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    // Tokio runtime starting
    let tk = runtime::Builder::new_multi_thread()
                            .build()
                            .unwrap();
    let n = A.len();

    for norm_row in 0..n {
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();

        let base_row = unsafe { &*A.as_ptr().offset(norm_row as isize) };

        for ind_row in (norm_row + 1)..n {
            let row_normalizing = unsafe { &mut *A.as_mut_ptr().offset(ind_row as isize) };
            let multiplier = row_normalizing[norm_row] / base_row[norm_row];

            let future = tk.spawn(async move {row_solver(row_normalizing, base_row, norm_row)});
            
            tasks.push(future);
            B[ind_row] -= multiplier * B[norm_row];
        }

        // Wait for all futures to finish
        for task in tasks {
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


// Use a scoped threadpool. Spawn a thread for every logical core in the machine
// and then for every row in the matrix, create a job to solve the row, and wait for all jobs to 
// finish before going to the next row of normalization.
// Performance is very good.
extern crate scoped_pool;
use scoped_pool::Pool;
fn gauss_solver_with_thread_pool(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    let total_threads = std::thread::available_parallelism().unwrap().get();
    let pool = Pool::new(total_threads);
    let n = A.len();

    for norm_row in 0..n {
        let base_row = unsafe { &*A.as_ptr().offset(norm_row as isize) };
        
        pool.scoped(|scope| {
            for ind_row in (norm_row + 1)..n {
                let row_normalizing = unsafe { &mut *A.as_mut_ptr().offset(ind_row as isize) };
                let multiplier = row_normalizing[norm_row] / base_row[norm_row];
                
                scope.execute(move || {row_solver(row_normalizing, base_row, norm_row);});
                
                B[ind_row] -= multiplier * B[norm_row];
            }
        });
    }

    pool.shutdown();

    for row in (0..n).rev() {
        X[row] = B[row];
        for col in ((row + 1)..n).rev() {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }
}

const MIN_CHUNK_SIZE:usize = 16;
fn gauss_solver_with_thread_pool_chunks(A: &mut Vec<Vec<f64>>, B: &mut Vec<f64>, X: &mut Vec<f64>) {
    let total_threads = std::thread::available_parallelism().unwrap().get();
    let pool = Pool::new(total_threads);
    let n = A.len();
    
    for norm_row in 0..n {
        let base_row = unsafe { &*A.as_ptr().offset(norm_row as isize) };
        
        // Calculate the size of each chunk
        let chunk_size = if (n - norm_row) / total_threads < MIN_CHUNK_SIZE {MIN_CHUNK_SIZE} else {(n - norm_row) / total_threads};
        let matrix = A[norm_row+1..].chunks_mut(chunk_size);
        let b_value = B[norm_row];
        let result = B[norm_row+1..].chunks_mut(chunk_size);

        pool.scoped(move |scope| {
            for (chunk, result_chunck) in matrix.zip(result) {
                scope.execute(move || {
                    rows_solver(chunk, base_row, norm_row, result_chunck, b_value);
                });
            }
        });
    }

    pool.shutdown();

    for row in (0..n).rev() {
        X[row] = B[row];
        for col in ((row + 1)..n).rev() {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }
}

fn rows_solver(
    matrix: &mut [Vec<f64>], 
    base_row: &[f64], 
    index: usize, 
    equality_vector:&mut [f64], 
    eq_vector_base_value:f64
) {
    for (i, current_row) in matrix.iter_mut().enumerate() {
        let multiplier = current_row[index] / base_row[index];
        for index_col in index..base_row.len() {
            current_row[index_col] -= multiplier * base_row[index_col];
        }
        equality_vector[i] -= multiplier * eq_vector_base_value;
    }
}