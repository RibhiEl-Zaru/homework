/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;



/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    
    assert!(!mat1.is_empty() && !mat2.is_empty(), "Matrixes need some data fam");
    
    let m = mat1[0].len();
    let col_count = mat2.len();
    println!("{}", m);
    println!("{}", col_count);
    assert!(m == col_count, "Row count of matrix 1 must equal col count of matrix 2");
    

    let n = mat1.len();
    let p = mat2[0].len();
    let mut vec: Matrix = vec![vec![0.;n]; p];
    for i in 0..n {
        for j in 0..p {
            let mut to_add = 0.0;
            for k in 0..m{
                to_add = to_add + mat1[i][k] * mat2[k][j];
                println!("{:?}", to_add);
            }
            vec[i][j] = to_add;    
            println!("{:?}", vec);
        }
    }
    println!("{:?}", vec);
    
    return vec.to_vec();
}
