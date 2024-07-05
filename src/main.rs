use std::env;

fn main() {
    // calling .collect() method an iterator produces a collection from the iterator
    // also note that the we need to annotate the variable here to let rust know
    // the type of the value we need to get from the collect method.

    let args: Vec<String> = env::args().collect();

    let query = &args.get(1);
    let filepath = &args.get(2);

    println!("Query = {}", query.unwrap_or(&String::from("")));
    println!("Filepath = {}", filepath.unwrap_or(&String::from("")));

}
