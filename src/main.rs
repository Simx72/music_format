use std::env;


fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        help();
    }

    let from = args.get(1)
        .expect("Missing 'from' parameter.");
    let to = args.get(2)
        .expect("Missing 'to' parameter.");
    
    println!("from: {}\t| to: {}", from, to);

    println!("the path of the file is: {}", from);


}


fn help() {
    println!("Hello world!
music_format helps changing audio formats easily.

Usage:
    music_format [from] [to]

Where:
    from    the path of the file/s to be converted
    to      the path where the converted will be saved to   
");
}