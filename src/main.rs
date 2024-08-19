//
// 1) declare two arrays in main, call dotprod(), return dotproduct
// 2) declare matrix [row_a X col_a] and multiply by matrix [col_a X col_b]
//
// Comments and thinking are in the first several commits. 
//
//

pub mod matmult;

fn main() {
    println!("Hello, world!");

    // 1) declare and initialize two equal length arrays
    let a : [f64; 7] = [5.0,4.0,6.0,3.0,7.0,2.0,1.0];
    let b : [f64; 7] = [2.0,2.0,2.0,2.0,2.0,2.0,2.0];

    println!("a is {:?}", a);
    println!("b is {:?}", b);

    // let c : (f64,u32) = dotprod(&a,&b);
    let c : f64 = matmult::dotprod(&a,&b).unwrap();

    println!("dotproduct of a and b is {:?}", c);

    //
    // 2) --- matrix math ------------------------------------
    //

    const ROW_A : usize = 3;
    const COL_A : usize = 4;
    const ROW_B : usize = COL_A;
    const COL_B : usize = 7;

    let dd2 = vec![vec![3.14159; COL_A]; ROW_A];

    // Different ways to initialize:
    let mut ee2 = vec! [vec![3.14159; COL_B]; ROW_B] ;
    //    [[1.0,2.0,3.0,4.0,5.0,6.0,7.0], [1.0,2.0,3.0,4.0,5.0,6.0,7.0], [1.0,2.0,3.0,4.0,5.0,6.0,7.0], [1.0,2.0,3.0,4.0,5.0,6.0,7.0]];
    for (i, row) in ee2.iter_mut().enumerate() { // i in 0..ee.len()-1 { // ROW_B-1 {
        for (j, val) in row.iter_mut().enumerate() { // j in 0..ee[0].len()-1 { // COL_B-1 {
            // ee[i][j] = ((2*i + (7+i) * (1+j))) as f64;
            *val = ((2*i + (7+i) * (1+j))) as f64;
        }
    }

    println!("dimensions of dd={:?}X{:?}",dd2.len(),dd2[0].len());
    println!("dimensions of ee2={:?}X{:?}",ee2.len(),ee2[0].len());

    // Declare the call and return variables
    let ret_mat = Some(matmult::mat_mult(&dd2, &ee2));

    // verify matrix is correct
    {
        println!("Verifying matrix");
        println!("matrix = {:?}", ret_mat);

    }

}

fn _example(width: usize, height: usize) { // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // Base 1d array
    let mut grid_raw = vec![0; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array `&mut [&mut [_]]`
    let grid = grid_base.as_mut_slice();

    // Accessing data
    grid[0][0] = 4;


    // https://stackoverflow.com/questions/50985003/is-there-a-short-notation-for-slicing-columns-from-a-2d-array-in-rust-using-onl
    let arr = [[1, 2, 3, 4]; 3];

    let iter = arr[0..2].iter().map(|s| &s[1..4]);

    for slice in iter {
        for x in slice {
            print!("{} ", x);
        }
        println!();
    }
}