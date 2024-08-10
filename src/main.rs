//
// 1) declare two arrays in main, call dotprod(), return dotproduct
// 2) declare matrix [row_a X col_a] and multiply by matrix [col_a X col_b]
//
// Comments and thinking are in the first commit. 
//
//

fn main() {
    println!("Hello, world!");

    // 1) declare and initialize two equal length arrays
    let a : [f64; 7] = [5.0,4.0,6.0,3.0,7.0,2.0,1.0];
    let b : [f64; 7] = [2.0,2.0,2.0,2.0,2.0,2.0,2.0];

    println!("length of a = {}",a.len()); // verify that len() is correct
    println!("a is {:?}", a);
    println!("b is {:?}", b);

    let c : (f64,u32) = dotprod(&a,&b);

    println!("dotproduct of a and b is {:?}", c);

    //
    // 2) --- matrix math ------------------------------------
    //

    const ROW_A : usize = 3;
    const COL_A : usize = 4;
    const ROW_B : usize = COL_A;
    const COL_B : usize = 7;

    let dd = vec![[3.14159 ; COL_A]; ROW_A];

    println!("dimensions of dd={:?}X{:?}",dd.len(),dd[0].len());

    let mut ee = vec! [[3.14159; COL_B]; ROW_B] ;
    //    [[1.0,2.0,3.0,4.0,5.0,6.0,7.0], [1.0,2.0,3.0,4.0,5.0,6.0,7.0], [1.0,2.0,3.0,4.0,5.0,6.0,7.0], [1.0,2.0,3.0,4.0,5.0,6.0,7.0]];
    for (i, row) in ee.iter_mut().enumerate() { // i in 0..ee.len()-1 { // ROW_B-1 {
        for (j,val) in row.iter_mut().enumerate() { // j in 0..ee[0].len()-1 { // COL_B-1 {
            // ee[i][j] = ((2*i + (7+i) * (1+j))) as f64;
            *val = ((2*i + (7+i) * (1+j))) as f64;
        }
    }
    println!("dimensions of ee={:?}X{:?}",ee.len(),ee[0].len());

    // multiply dd * ee = ff ; this needs to be a function, of course! ---
    //
    let mat_a = dd.clone();
    let mat_b = ee.clone();
    let mut ff = vec![[0.0; COL_B]; ROW_A];

    println!("Calling mat_mult()");
    mat_mult(&mat_a, &mat_b, &mut ff);

    // verify matrix is correct
    {
        println!("Verifying matrix");
        // let zz: [[f64; 4]; 4] = dd; // works
        let zz = ff.clone(); // works and I don't need to worry about the type or ownership

        println!("matrix = {:?}", zz);
        // println!("matrix[3][3] {:?}", zz[2][2]); // this can panic if you go out of bounds
        // println!("matrix[2][3] {:?}", zz[ROW_A-1][COL_A-1]); 

        // Now I need to get the column slices of dd (rows are trivial)
        // println!("third row of zz = {:?}",zz[2]);
        //get a column // 
        let _col = zz // r/learnrust on reddit, nboro94 Easy way to slice 2d arrays in rust for beginners, get columns and square slices from your arrays
            .iter()
            .map(|s| s.iter().nth(2).unwrap())
            .collect::<Vec<_>>();
        // println!("the third column is: {:?}", col);
        // these don't work :-(
        // println!("third col of zz = {:?}",zz.get(2)); // zz[0..zz.len()-1][zz[0].len()-1]);
    
    }

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
// This function is still reliant on fixed size arrays!!! How can I make it accept (at least)
// preallocated matrices ... and at best, preallocated lhs and pushed elements onto an
// empty matrix?
//
fn mat_mult(mat_a : &Vec<[f64; 4]>, mat_b : &Vec<[f64; 7]>, ret_m : &mut Vec<[f64; 7]>) -> () {
    for (i, row_i) in mat_a.iter().enumerate() { // no. rows in mat_a
        println!("in mat_mult: {}th row of mat_a = {:?}",i, row_i);
        for j in 0..mat_b[0].len() { // no. cols in B
            let col_j = mat_b
                .iter()
                .map(|s| *s.iter().nth(j).unwrap())
                .collect::<Vec<_>>();
            println!("{}th colum of mat_b = {:?}", j, col_j);
            ret_m[i][j] = dotprod(row_i,&col_j).0;
        }
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
}

fn dotprod(a:&[f64], b:&[f64]) ->  (f64, u32) {

    // return if the vectors are not the same length
    if a.len() != b.len() {
        return (-99.0, 1);
    }

    // return if vectors are 0-length
    if  a.len() < 1 || b.len() < 1 {
        return (-99.0,2);
    }

    let mut atimesb : f64 = 0.;

    for i in 0..a.len() {
        atimesb += a[i]*b[i];
    }

    (atimesb,0) // 0 == all's well
}