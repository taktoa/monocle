use ndarray::{Array1, Array2};
use std::error::Error;
use linfa::prelude::*;
use linfa_elasticnet::ElasticNet;

// A x = y

// minimize ||Ax - y||_2

// minimize ||Ax - y||_2 + lambda * ||x||_1

// minimize ||Ax - y||_2 + lambda * ||Bx||_1
// minimize ||(A * B^-1)x - y||_2 + lambda * ||x||_1



pub fn dct_1d(vec: &Array1<f64>) -> Array1<f64> {
    // y_k = sum_0^(n - 1) x_n cos((pi / N) * (n + 0.5) * k)
    let size = vec.shape()[0];
    let mut result = Array1::<f64>::zeros(size);
    for k in 0 .. size {
        let mut value = 0.0;
        for n in 0 .. size {
            value += vec[n] * f64::cos((std::f64::consts::PI / (size as f64))
                                       * ((n as f64) + 0.5) * (k as f64));
        }
        result[k] = value;
    }
    result
}

// pub fn idct_1D(vec: &Array1<f64>) -> Array1<f64> {
//     // y_k = (x_0 / sqrt(2) + sum_1^(n - 1) x_n cos((pi / N) * (k + 0.5) * n))
//     //       * sqrt(2 / N)
//     let size = vec.shape()[0];
//     let mut result = Array1::<f64>::zeros(size);
//     for k in 0 .. size {
//         let mut value = vec[0] / 2.0;
//         for n in 1 .. size {
//             value += vec[n] * f64::cos((std::f64::consts::PI / (size as f64))
//                                        * ((k as f64) + 0.5) * (n as f64));
//         }
//         result[k] = value;
//     }
//     result
// }

// pub fn dct_matrix(dimension: usize) -> Array2<f64> {
//     let mut result = Array2::<f64>::zeros((dimension, dimension));
//     let sqrt_dimension = f64::sqrt(dimension as f64);
//     for ((i, j), element) in result.indexed_iter_mut() {
//         if i == 0 {
//             *element = omega.powi((i * j) % dimension) / sqrt_dimension;
//         }
//     }
//     result
// }

// Dataset<f64, f64>
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cosine = Array1::<f64>::zeros(100);
    for i in 0 .. cosine.shape()[0] {
        cosine[i] = f64::cos((i as f64) / 25.0);
    }

    let dct = dct_1d(&cosine);
    println!("DEBUG: {:?}", dct);


    // let nsamples = 1000;
    // let nfeatures = 2000;
    // let train = linfa::Dataset::new(Array2::<f64>::ones((nsamples, nfeatures)),
    //                                 Array2::<f64>::ones((nsamples, 1)));
    //
    // // train pure LASSO model with 0.3 penalty
    // let model = ElasticNet::<f64>::params()
    //     .penalty(0.3)
    //     .l1_ratio(1.0)
    //     .with_intercept(false)
    //     .fit(&train)?;
    //
    // println!("intercept:  {}", model.intercept());
    // println!("params: {}", model.hyperplane());
    //
    // println!("z score: {:?}", model.z_score());

    Ok(())
}

// pub fn main() -> Result<(), Box<dyn Error>> {
//     let mut builder = ll::Builder::new();
//     let data = ll::util::TrainingInput::from_dense_features(
//         vec![100.0, 200.0, 100.0],
//         vec![vec![-1.5, 2.5], vec![1.0, -3.5], vec![-1.0, -3.2]],
//     ).unwrap();
//     builder.problem().input_data(data).bias(-1.0);
//     builder.parameters()
//         .solver_type(ll::SolverType::L2R_LR)
//         .stopping_criterion(0.01)
//         .constraints_violation_cost(1.0)
//         .regression_loss_sensitivity(0.1);
//     // .cost_penalty_weights(Vec::<f64>::new())
//     // .cost_penalty_labels(Vec::<i32>::new())
//     // .initial_solutions(Vec::<f64>::new())
//     let model = builder.build_model()?;
//     println!("f([-0.8, 0.5]) = {}",
//              model.predict(
//                  ll::util::PredictionInput::from_dense_features(
//                      vec![-0.8, 0.5, 0.3]).unwrap()).unwrap());
//     Ok(())
// }

// pub fn main() -> Result<(), Box<dyn Error>> {
//     let mut builder = ll::Builder::new();
//     let data = ll::util::TrainingInput::from_dense_features(
//         // vec![100.0, 200.0, 100.0],
//         vec![100.0, 200.0],
//         // vec![vec![-1.5, 2.5], vec![1.0, -3.5], vec![-1.0, -3.2]],
//         vec![vec![-1.5, 1.0, -1.0], vec![2.5, -3.5, -3.2]],
//     ).unwrap();
//     println!("DEBUG: {}", data.len_features());
//     builder.problem().input_data(data).bias(-1.0);
//     let model = builder.build_model()?;
//     println!("f([-0.8, 0.5]) = {}",
//              model.predict(
//                  ll::util::PredictionInput::from_dense_features(
//                      vec![-0.8, 0.5]).unwrap()).unwrap());
//     Ok(())
// }
