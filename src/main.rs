use std::convert::TryFrom;
use std::io::{self, Write};
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
    /// Input YML file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /*
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    */
}

#[derive(Debug)]
#[derive(Clone)]
struct Table {
    // A vector of columns (vectors) of rows. Each column represents an implicant set. 
    entries: Vec<Vec<Row>>,
    // the SOP min-term list
    all_implicants: Vec<u32>,
    // bit size of the data
    bit_size: usize,
}

#[derive(Debug)]
#[derive(Clone)]
struct Row {
    // vecor of the binary 
    bin: Vec<u32>,
    // the number of ones in the row
    ones: u32,
    // the implicant(s) within the row
    implicants: Vec<u32>
}


fn main() -> std::io::Result<()> {

    let args = Cli::from_args();

    println!("{:?}", args.input);

    let sop: Vec<u32> = vec![27, 15, 16, 29, 28, 0,4,6,8];

    // Initialize the table.
    let mut table = initialize_table(& sop);

    // Is this a case for references? Where we enter a reference and it is in the main 
    // function that we write to the table? It is something to consider. Actualy, this is 
    // probably a case for implying methods to the struct.
    table = initial_comparison(table);


    // Do the comparison until no more implicants can be found.
    // A result of "none" will be returned if there are no more implicants
    let mut result = comparison(table.clone());
    while result.is_some() {
        table = result.unwrap();
        result = comparison(table.clone());
    };

    // Get the prime/essential implicants
    let prime_imps = reduce_to_prime_implicants(table);

    // pretty print the results
    table_print(prime_imps); 
    
    Ok(())

}

fn table_print(table: Vec<Row>) {

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut handle = io::BufWriter::new(stdout_lock);

    write!(handle, "\n").expect("write error");
    
    for column in 0..table.len() {
        write!(handle, "|").expect("write error");
        for row in 0..table[0].bin.len() {
            if table[column].bin[row] == 2 {
                write!(handle, " -").expect("write error");
            } else {
                write!(handle, " {}", table[column].bin[row]).expect("write error");
            }
            
        }
        write!(handle, " |\n").expect("write error");

    }

}

// Reduce the table to prime, essential implicants
fn reduce_to_prime_implicants (table: Table) -> Vec<Row> {

    // imps contains a vector of the found implicants; primed with the last row, last column
    let mut imps: Vec<u32> = Vec::new();
    // Get the last column
    let mut end_column: usize = table.entries.len() -1;
    // Get the last column, minus the already primed imps.
    let mut end_row: usize = table.entries.last().unwrap().len() -1;
    // Vector of the Rows that are prime implicants, primed with the first one
    let mut prime_imps: Vec<Row> = Vec::new();


    // Loop until all of the imps have been found.
    loop {
        // Check each implicant entry to see if it is already included
        for i in 0..table.entries[end_column][end_row].implicants.len() {
            // If not, then add all of the implicants in the entry and push the Row
            if ! imps.contains(& table.entries[end_column][end_row].implicants[i]) {
                imps.extend(table.entries[end_column][end_row].implicants.clone());
                prime_imps.push(table.entries[end_column][end_row].clone());
            }
        }

        // Check to see if we are done
        if vec_in( & imps, & table.all_implicants) {
            break;
        }

        // Decriment the counters
        if end_row == 0 {
            end_column -= 1;
            end_row = table.entries[end_column].len() -1;
        } else {
            end_row -= 1;
        }
    }
  
    
    // Return prime implicants
    prime_imps 

}

// Check to see if vec_b is contained within vec_a
fn vec_in (vec_a: & Vec<u32>, vec_b: & Vec<u32>) -> bool {
    for i in 0..vec_b.len() {
        if ! vec_a.contains(& vec_b[i]) {
            return false 
        }
    }

    true
}

// If there is a dublicate, return true. Else, return false
fn implicant_duplicate (imps_a: & Vec<Row>, imps_b: & Vec<u32>) -> bool {
    // Test to see if the implicant has already been found 
    for b in 0..imps_a.len() {
        if vec_in(& imps_a[b].implicants, & imps_b) {
            return true;
        }
    }

    return false;
}


// Compare the implicants
fn comparison (mut table: Table) -> Option<Table> {

    let mut bin: Vec<u32> = Vec::new();
    let mut diffs: u32 = 0;
    let mut imps: Vec<Row> = Vec::new();
    let mut temp_implicants: Vec<u32>;
    let mut dashes1: Vec<usize> = Vec::new();
    let mut dashes2: Vec<usize> = Vec::new();
    // For lazyness clone the set of data needed to increase readability...maybe should
    // be switched to refernces and dereferences
    let work_set: Vec<Row> = table.entries.last().unwrap().clone(); 

    // For each Row in the last vector in table.entries
    for i in 0..(work_set.len()) {
        // Find the indexes of the dashes
        for n in 0..(work_set[i].bin.len()) {
            if work_set[i].bin[n] == 2 {
                dashes1.push(n);
            }
        }

    
        // For each Row that has one more "one" than the above Row
        for a in i..(work_set.len()) {

            dashes2.clear();

            // This could be put in a function
            if work_set[a].ones == work_set[i].ones  + 1 {
                // Get the indexes of the dashes 
                for n in 0..(work_set[a].bin.len()) {
                    if work_set[a].bin[n] == 2 {
                        dashes2.push(n);
                    }
                }


                // Compare the indexes of the dashes. If they are not the same, pass along
                if dashes1 != dashes2 {
                    continue;
                }

                // Compare the differences
                for n in 0..(work_set[i].bin.len()) {
                    if work_set[i].bin[n] == work_set[a].bin[n] {
                        bin.push(work_set[i].bin[n]);
                    } else {
                        bin.push(2);
                        diffs += 1;
                    }
                    // Check to see if the difference is greater than one
                    if diffs > 1 {
                        break;
                    }
                }

                // Check to see if the differences is greater than one
                if diffs > 1 {
                    continue;
                }
                
                // Put together the base implicants of the candidate new implicant
                temp_implicants = [work_set[i].implicants.clone(), work_set[a].implicants.clone()].concat();
            
                
                // LOgic not right!!!!!!
                // Test to see if the implicant has already been found 
                // if Yes, Move on!
                if implicant_duplicate(& imps, & temp_implicants) {
                    temp_implicants.clear();
                    bin.clear();
                    diffs = 0;

                    continue;
                }
                
                // Push the row to the imps
                imps.push(Row {
                    bin: bin.clone(),
                    ones: work_set[i].ones,
                    implicants: temp_implicants.clone()
                });
               
                // clear out the variables
                temp_implicants.clear();
                bin.clear();
                diffs = 0;

            }
            // If the number of ones is greater than one differnt, break the loop
            else if work_set[a].ones >= work_set[i].ones  + 1 {
                break;
            }

        }

        // Reset bin, diffs, dashes
        dashes1.clear();

    }

    // return the result wrapped in an option.
    if imps.len() == 0 {
        None
    } else {
        table.entries.push(imps);
        Some(table)
    }

}



// Do the inital comparison throwing in the first set of dashes (2's...because u32 doesn't
// include dashes)
fn initial_comparison (mut table: Table) -> Table {

    // imps is a vector of rows that houses the new column of implicants
    let mut imps: Vec<Row> = Vec::new();
    // num_dashes is a u32 that contains the number of dashes (don't cares) in a row. If 
    // there is more or less than one then the rows cannot be combined.
    let mut num_dashes: u32 = 0;
    // temp is a vector of binary implicants.
    let mut temp: Vec<u32> = Vec::new();

    // iterate over each entry in the array
    for i in 0..(table.entries[0].len()) {
        // For each array entry, compare it to all the entries following it. 
        for n in i..table.entries[0].len() {

            // Only compare the entries that have one more "one" in it.
            if table.entries[0][n].ones == table.entries[0][i].ones +  1 {

                // Compare each entry
                for x in 0..(table.entries[0][i].bin.len()) {
                    // if the entries match, push the entry to the temp vector
                    if table.entries[0][i].bin[x] == table.entries[0][n].bin[x] {
                        temp.push(table.entries[0][i].bin[x]);

                    // if they don't match, increment the number of dashes and push 2
                    } else {
                        num_dashes += 1;
                        temp.push(2);
                    }

                    // Check to see if there is more than one dash and break if so
                    if num_dashes > 1 {
                        break;
                    }
                }

                // if all of the bits have been compared, and there is only one dash, push 
                // the new implicant into imps
                if num_dashes == 1 {

                    imps.push(Row {
                        bin: temp.clone(),
                        ones: table.entries[0][n].ones,
                        implicants: [table.entries[0][n].implicants.clone(), table.entries[0][i].implicants.clone()].concat()
                    })
                }
               
                // Rest for the next iteration
                num_dashes = 0;
                temp.clear();
            }
            // check to see if the loop ieterations have passed the one different "one"
            else if table.entries[0][n].ones > table.entries[0][i].ones +  1 {
                break;
            }
        }
    }

    // Push the new implications into another column of the entries table.
    if ! imps.is_empty() {
        table.entries.push(imps);
    }
    // return it!
    table
}



// Quickly sort the truth table entries by the number of ones they have in them
fn quick_sort (table: Vec<Row>) -> Vec<Row> {

    // If the array has a length less than or equal to one then it is already sorted
    if & table.len() <= & 1 {
        return table 
    }

    // delare the three vectors
    let mut smaller: Vec<Row> = Vec::new();
    let mut equal: Vec<Row> = Vec::new();
    let mut larger: Vec<Row> = Vec::new();

    // Get the pivot in the middle of the array
    // The ends are bad choices because often the list is already almost sorted
    let pivot = & table[(& table.len()/2)].ones;

    // Iterate and devide the values into the respective vectors
    for x in & table {
        if x.ones < * pivot {
            smaller.push(x.clone());
        } else if x.ones == * pivot {
            equal.push(x.clone());
        } else {
            larger.push(x.clone());
        }
    }

    // return recursivly.
    [quick_sort(smaller), equal, quick_sort(larger)].concat()
    
}

fn initialize_table (sop: & Vec<u32>) -> Table {
    // Get the bit size needed to hold all of the SOP implicants
    let bit_size = max_n(&sop);

    // initialze a temporary row
    let mut the_row = Row {
        bin: vec![0,0,0,0],
        ones: 0,
        implicants: vec![0],
    };

    // initialize a vector of row
    let mut vec_of_rows: Vec<Row> = Vec::new();
   
    // Throw a row into the vector of rows
    for i in sop {
        the_row.bin = dec_2_bin_vec(i, &bit_size);
        the_row.ones = sum_bin_vec(& the_row.bin);
        the_row.implicants = vec![*i];

        vec_of_rows.push(the_row.clone());
    }

    // Quick sort the rows by the number of ones
    vec_of_rows = quick_sort(vec_of_rows);

    // Create the table
    let the_table = Table {
        entries: vec![vec_of_rows],
        all_implicants: sop.clone(),
        bit_size: bit_size,
    };


    // Return it!!
    the_table
}   



fn sum_bin_vec (bin: & Vec<u32>) -> u32 {
    
    bin.iter().sum::<u32>()

}

fn dec_2_bin_vec (dec: & u32, bit_size: & usize) -> Vec<u32> {
    let mut temp: Vec<u32> = Vec::new();

    let mut q = dec.clone();


    // Iterate through each value and push a 1 or 0 respectivly
    while q > 0 {
        if q%2 == 1 {
            // if there is a remainder, push 1
            temp.push(1)
        } else {
            // if there is no remainder, push 0
            temp.push(0)
        }
        
        q = q/2;
    }

    // Fill in extra zeros as needed
    while temp.len() < * bit_size{
        temp.push(0);
    }

    // reverse the values to put them in the correct order
    temp.reverse();

    // return temp
    temp
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


