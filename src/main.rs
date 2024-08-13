//
// 1) declare two arrays in main, call dotprod(), return dotproduct
// 2) declare matrix [row_a X col_a] and multiply by matrix [col_a X col_b]
//
// Comments and thinking are in the first several commits. 
//
//

fn main() {
    println!("Hello, world!");

    // 1) declare and initialize two equal length arrays
    let a : [f64; 7] = [5.0,4.0,6.0,3.0,7.0,2.0,1.0];
    let b : [f64; 7] = [2.0,2.0,2.0,2.0,2.0,2.0,2.0];

    println!("a is {:?}", a);
    println!("b is {:?}", b);

    // let c : (f64,u32) = dotprod(&a,&b);
    let c : f64 = dotprod(&a,&b).unwrap();

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
    let mat_a2 = dd2.clone(); // don't copy this ... it's read only in the function anyway.
    let mat_b2 = ee2.clone(); // make sure to implicitely declare INTENT w/the code
    // let mut ff = vec![[0.0; COL_B]; ROW_A];

    // mat_mult2(&mat_a2, &mat_b2, &mut ff); // These both return the same matrix
    let ret_mat = Some(mat_mult2(&mat_a2, &mat_b2));
    // note: no need to declare type of size; Rust compiler should figure this out.

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

// 
// Return type should not be () w/"output" being a fortran like "intent inout" that is overwritten.
// Instead, make a clean output variable and pass it back to the calling function.
//
// JO Your input is dynamic ... but output has a specified size. Not good!
//
// fn mat_mult2(mat_a : &Vec<Vec<f64>>, mat_b : &Vec<Vec<f64>>, ret_m : &mut Vec<[f64; 7]>) -> () {
fn mat_mult2(mat_a : &Vec<Vec<f64>>, mat_b : &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {

    let mut tmp_mat : Vec<Vec<f64>> = vec![vec![0.0; mat_b[0].len()]; mat_a.len()];
    for (i, row_i) in mat_a.iter().enumerate() { // no. rows in mat_a
        println!("in mat_mult: {}th row of mat_a = {:?}",i, row_i);
        for j in 0..mat_b[0].len() { // no. cols in B
            let col_j = mat_b
                .iter()
                .map(|s| *s.iter().nth(j).unwrap())
                .collect::<Vec<_>>();
            println!("{}th colum of mat_b = {:?}", j, col_j);
            tmp_mat[i][j] = dotprod(row_i,&col_j).unwrap();
        }
    }
 
    //If tests on tmp_mat are OK, then return tmp_mat, else return "None"
    //if {
    let ret_m : Option<Vec<Vec<f64>>> = Some(tmp_mat);
    //} else {
    //  let mut ret_m : Option<Vec<Vec<f64>>> = None;
    //}
    return ret_m;

    // JO: you should have a &Vec<Vec<f64>> here.
    //  wmy: passing back ... actually not sure what I'm passing back!? Is option above, OK?
    // JO: you have to pass back an owned variable.
    //  wmy: not sure if I did this -- but the code works.
}

fn dotprod(a:&[f64], b:&[f64]) -> Option<f64> { // ->  (f64, u32) { //

    // Rust book ... options ... return value + flags, not ( ..., flag) ... 
    // 
    let mut ret_val : Option<f64> = None;

    // return if the vectors are not the same length
    if a.len() != b.len() {
        return ret_val // (-99.0, 1);
    }

    // return if vectors are 0-length
    if  a.len() < 1 || b.len() < 1 {
        return ret_val // (-99.0,2);
    }

    let mut atimesb : f64 = 0.;

    for i in 0..a.len() {
        atimesb += a[i]*b[i];
    }

    // (atimesb,0) // 0 == all's well
    ret_val = Some(atimesb);
    return ret_val;


    // return should look like this:
    // fn () -> Option<f64> 
    //

}