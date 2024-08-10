//
// declare two arrays in main, call dotprod(), return dotproduct
//

fn main() {
    println!("Hello, world!");

    // declare and initialize two equal length arrays
    let a : [f64; 7] = [5.0,4.0,6.0,3.0,7.0,2.0,1.0];
    let b : [f64; 7] = [2.0,2.0,2.0,2.0,2.0,2.0,2.0];

    println!("length of a = {}",a.len()); // verify that len() is correct
    println!("a is {:?}", a);
    println!("b is {:?}", b);

    let c : (f64,u32) = dotprod(&a,&b);

    println!("dotproduct of a and b is {:?}", c);

    //
    // --- matrix math ------------------------------------
    //

    let mut aa = // implicit type [[f64; 4]; 4], works
     [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]];
 
    // aa.push(5.0); // will fail!

    const ROW_A : usize = 3;
    const COL_A : usize = 4;
    const ROW_B : usize = COL_A;
    const COL_B : usize = 7;

    //
    let mut bb = vec![vec![0.0; COL_A]; ROW_A]; // passes compiler ... but can't easily use :-()
    bb[2][2] = 7.0;
    bb[ROW_A-1][COL_A-1] = 99.0; // hmmmm ... why doesn't this panic!?
    // for example, can't access like this:
    // bb = [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0]];
    bb[2] = vec![3.1; COL_A];
    bb = vec![vec![1.0, 2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0, 8.0], vec![9.0, 10.0, 11.0, 12.0]];
    
    // but you _can_ do this, again w/mostly implicit typing
    let mut cc = vec![[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]];

    //
    // *** I don't understand the difference between the initializations of aa, bb and cc ***
    //
    // stack vs. heap ... stack reqs predifined sizes (bytes!) at ocmpile time! ... and can't use too much bytes.
    // vec will always know size ahead of time
    // [..] are continuous memory ... if you know the size, it's in the stack. (line 24) is actual fixed known data. 
    // (line 39) re: vec! ... pointers are in stack and interact w/heap ... vec! is a pointer ... can add values.
    // you cannot modify the pointer -- mut refers to data only.
    //
    // *** In below: WHY does Rust require explicit initialization if I declared type AND size?

    // But this seems the most useable 'base' rust way to make matrices
    // let mut dd: [[f64; COL_A]; ROW_A] = // this is the most understandable to me
    let mut dd = vec![[3.14159 ; COL_A]; ROW_A];
    //    vec![[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0]];
    //for i in 0..ROW_A-1 { // dd.len()-1 { //
    //    for j in 0..COL_A-1 { // dd[0].len()-1 { // this notation is clunky b/c you require explicit size
    //       dd[i][j] = ((i + (1+i) * (1+j))) as f64;
    //    }
    //} // next loop does exactly the same thing:
    for (i, row) in dd.iter_mut().enumerate() {
        for (j , val) in row.iter_mut().enumerate() {
            // dd[i][j] = ((i + (1+i) * (1+j))) as f64;
                   *val = ((i + (1+i) * (1+j))) as f64;
        }
    }

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

    //for i in 0..dd.len() { // no. rows in dd
    //    let row_i = dd[i];
    for (i, row_i) in mat_a.iter().enumerate() {
        // for j in 0..ee[0].len() { // no. cols in mat_a
        for (j,_value) in mat_b[0].iter().enumerate() { // no. cols in mat_b
            let col_j = mat_b //
                .iter()
                .map(|s| *s.iter().nth(j).unwrap())
                .collect::<Vec<_>>(); 
            // ff[i][j] = dotprod(&row_i,&col_j).0;
            ff[i][j] = dotprod(row_i,&col_j).0;
        }
    }

    println!("Calling mat_mult()");
    mat_mult(ROW_A, COL_A, &mat_a, ROW_B, COL_B, &mat_b, &mut ff);

    // verify matrix is correct
    {
        println!("Verifying matrix");
        // let zz: [[f64; 4]; 4] = dd; // works
        let zz = ff.clone(); // works and I don't need to worry about the type or ownership

        println!("matrix = {:?}", zz);
        // println!("matrix[3][3] {:?}", zz[2][2]); // this can panic if you go out of bounds
        // println!("matrix[2][3] {:?}", zz[ROW_A-1][COL_A-1]); 

         // Hint: Removing variables from the workspace is poor practice.  In the parallel
         // realm you might free it prior to it's full scope ... let rust manage all memory.
         // You _only_ concentrate on ownership!

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

    // these lines are to remove the rustup warnings about mut in declaration
    aa[2][2] = 99.0;
    bb[2][2] = 99.0;
    cc[2][2] = 99.0;
    // dd[2][2] = 99.0;

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
fn mat_mult(_row_a : usize, _col_a : usize, mat_a : &Vec<[f64; 4]>, _row_b : usize, _col_b : usize, mat_b : &Vec<[f64; 7]>, ret_m : &mut Vec<[f64; 7]>) -> () {
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
    // I'm sure the dotproduct is a library function but I'll try to write anyway!

    // println!("in dotprod(): length of a is {}, length of b is {}", a.len(), b.len());

    // return if the veectors are not the same length
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
        // println!("a={}, b={}, a*b={}, sum={}",a[i],b[i],a[i]*b[i], atimesb);
    }

    (atimesb,0) // 0 == all's well
}