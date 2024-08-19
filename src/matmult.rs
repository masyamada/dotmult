pub fn mat_mult(mat_a : &Vec<Vec<f64>>, mat_b : &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {

    let mut tmp_mat : Vec<Vec<f64>> = vec![vec![0.0; mat_b[0].len()]; mat_a.len()];
    for (i, row_i) in mat_a.iter().enumerate() { // no. rows in mat_a
        println!("in mat_mult: {}th row of mat_a = {:?}",i, row_i);
        for j in 0..mat_b[0].len() { // no. cols in B
            let col_j = mat_b
                .iter()
                .map(|s| *s.iter().nth(j).unwrap())
                .collect::<Vec<_>>();
            println!("{}th colum of mat_b = {:?}", j, col_j);

            // 1) this works if called w/good arguments-- but unwrap() destroys self and can panic w/out warning
            //    use unwrap_or(), unwrap_or_else(), unwrap_or_default()
            tmp_mat[i][j] = dotprod(row_i,&col_j).expect("dotprod() is unassignable");
            
            // 2) I don't understand this; but it seems to work
            let x : Option<f64> = dotprod(row_i,&col_j);
            match x {
                Some(x) => {  tmp_mat[i][j] = x },
                None => { println!("problem in mat_mult2 at {} {}", i, j) },
            };
            
            // 3) Try expect() -- expect() will destroy the original Option<f64>, but will print an error message
            tmp_mat[i][j] = x.expect("dotprod() returned None");
            // or do it in a single line
            tmp_mat[i][j] = dotprod(row_i,&col_j).expect("dotprod() returned None"); // but i and j are out of context

            // try "if let" syntax (no need to typecheck or check for NULL ... Option type takes care of that!)
            let x : Option<f64> = dotprod(row_i,&col_j);
            if  x.is_some() {
                tmp_mat[i][j] = x.expect("in mat_mult2, dotprod() is unassignable");
            } else {
                println!("dotproduct returned None for ({},{}),", i, j);
            }

            // Can I use an enum to send back an actual error message saying what, exactly went wrong?
            // see Treor Sullivan playlist https://www.youtube.com/watch?v=z8k_EViGC10
            // 

            // finally, rewrite dotprod to return a Result, which has an error message in it (?)
            // Option is for functions that may, but might not, return a value
            // Result is for functions that are expected to succeed but might fail (w/error message)
            //
            // see rust documentation; Result is used exactly like Option; nothing to learn and 
            // better to use Option in this context
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


pub fn dotprod(a:&[f64], b:&[f64]) -> Option<f64> { // ->  (f64, u32) { //

    let mut ret_val : Option<f64> = None;

    // return if the vectors are not the same length
    if a.len() != b.len() {
        return ret_val
    }

    // return if vectors are 0-length
    if  a.len() < 1 || b.len() < 1 {
        return ret_val
    }

    // sum(a .* b)
    let mut atimesb : f64 = 0.;
    for i in 0..a.len() {
        atimesb += a[i]*b[i];
    }

    ret_val = Some(atimesb);
    return ret_val;

}

