// use std::io::{self, Write};
use std::fs::File;

mod table;

use table::initialize_table;
use table::initial_comparison;
use table::comparison;
use table::reduce_to_prime_implicants;
use table::table_print;


extern crate csv;
extern crate clap;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {

    let args = App::new("QMCA")
                      .version("0.1.1")
                      .author("Micah Tseng - tseng.micah@gmail.com")
                      .about("An implementation of the Quine McCluskey Algorithm in Rust")
                      .arg(Arg::with_name("SOP")
                           .short("s")
                           .long("sop")
                           .help("Input CSV file containing the SOP terms in the first line")
                           .value_name("FILE")
                           .required(true)
                           .takes_value(true))
                      .get_matches();

    let sop_file = args.value_of("SOP").unwrap_or("sop.csv");

    let sop = get_sop(sop_file.clone()).unwrap();

    // let sop: Vec<u32> = vec![27, 15, 16, 29, 28, 0,4,6,8];

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

// Get the SOP terms from the CSV file.
fn get_sop(input_path: & str) -> Result<Vec<u32>, csv::Error> {
    let mut sop: Vec<u32> = Vec::new();

    // Open the file
    let file = File::open(input_path)?;

    // Create the file reader and then read the csv from the file
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // Grab the results and spit them into a Vec<u32>
    for result in rdr.records() {
        let record = result?;
        for entry in record.iter() {
            sop.push(entry.parse::<u32>().unwrap());
        }
    }

    Ok(sop)
}


