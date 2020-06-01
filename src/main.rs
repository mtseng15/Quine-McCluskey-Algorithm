use std::convert::TryFrom;

fn main() {
    let sop: Vec<u32> = vec![0,4,5,6,7,8,12,15];

    // Find the max number of binary digits
    let n2 = max_n(&sop);

    let mut tt = to_bin_vec(&sop, n2);

    tt = push_sum_of_ones(tt);

    tt = quick_sort_ones(tt);

    tt = pop_sum_of_ones(tt);
   
    println!("{:?}", tt);

    

}


// Append the number of zeros to the end of each of the rows in the truth table
fn push_sum_of_ones (mut array: Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    for x in array.iter_mut() {
        x.push( x.iter().sum());
    }
    
    array
}

// Pop the sum of the zeros from the end of each of the rows in the truth table
fn pop_sum_of_ones (mut array: Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    for x in array.iter_mut() {
        x.pop();
    }
    
    array
}
// Quickly sort the truth table entries by the number of ones they have in them
fn quick_sort_ones (array: Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    // If the array has a length less than or equal to one then it is already sorted
    if & array.len() <= & 1 {
        return array
    }

    // delare the three vectors
    let mut smaller: Vec<Vec<u32>> = Vec::new();
    let mut equal: Vec<Vec<u32>> = Vec::new();
    let mut larger: Vec<Vec<u32>> = Vec::new();

    // Get the pivot in the middle of the array
    // The ends are bad choices because often the list is already almost sorted
    let pivot = & array[(& array.len()/2)].last();

    // Iterate and devide the values into the respective vectors
    for x in & array {
        if x.last() < * pivot {
            smaller.push(x.clone());
        } else if x.last() == * pivot {
            equal.push(x.clone());
        } else {
            larger.push(x.clone());
        }
    }

    // return recursivly.
    [quick_sort_ones(smaller), equal, quick_sort_ones(larger)].concat()
    
}


fn to_bin_vec (sop: & Vec<u32>, n2: usize) -> Vec<Vec<u32>> {
    let mut tt: Vec<Vec<u32>> = Vec::new();
    let mut temp: Vec<u32> = Vec::new();

    let mut q: u32;
    for i in  sop {

        // Iterate through each value and push a 1 or 0 respectivly
        q = * i;
        while q > 0 {
            if q%2 == 1 {
                temp.push(1)
            } else {
                temp.push(0)
            }

            q = q/2;
        }

        // Fill in extra zeros as needed
        while temp.len() < n2 {
            temp.push(0);
        }

        // reverse the values to put them in the correct order
        temp.reverse();
        // clone and push the values to the truth table
        tt.push(temp.clone());
        // Clear the temp
        temp.clear();
    }

    tt

}

// Find the needed number of bits
fn max_n ( sop: & Vec<u32> ) -> usize {

    // Find the max value in the SOP value
    let mut max = & sop[0];

    for i in sop.iter() {
        if i > max {
            max = & i;
        }
    }

    // Find the number of binary digits needed
    let mut int_value =  2; // the non remaining value
    let mut n2 = 1; // the number of digits

    while int_value <= *max {
        int_value =  int_value*2;
        n2 += 1;
    }

    // Retrn a usize
    usize::try_from(n2).unwrap()
}


