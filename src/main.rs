use std::env;


fn main() {
    // getting arguments
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        help();
    }

    let from = args.get(1)
        .expect("Missing 'from' parameter.");
    let to = args.get(2)
        .expect("Missing 'to' parameter.");
    
    println!("from: {}\t| to: {}", from, to);

    // processing arguments
    let cwd = env::current_dir().expect("Error: Current directory does not exist or there are insufficient permissions to access it.");

    let from = cwd.join(from);
    let to = cwd.join(to);

    println!("from: {} \t| to: {}", from.display(), to.display());

    




}


fn help() {
    println!("Hello world!
music_format helps changing audio formats easily.

Usage:
    music_format [from] [to]

Where:
    from    the path of the file (or folder with files) to be converted
    to      the path where the converted will be saved to   
");
}