use clap::Parser;

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[command(arg_required_else_help = true, author="Cameron", version, about="This is a simple clone of grep", long_about = None)]
struct Cli {
    // three '/' are read by clap to make descriptions
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    // flags
    #[arg(short, long)]
    verbose: bool, // verbose
    /// only print the first match instance
    #[arg(short, long)]
    first: bool, // implemented!
    /// ignore case of pattern 
    #[arg(short, long)]
    ignore_case: bool, 
    /// print count of matches 
    #[arg(short, long)]
    count: bool, //implemented!
    /// print all non-matching lines
    #[arg(short, long)]
    reverse: bool, //  implemented!
}

fn main() {
    /*/
    let pattern_in = std::env::args().nth(1).expect("no pattern given");
    let path_in = std::env::args().nth(2).expect("no path given");

    let args = Cli{
        pattern: pattern_in,
        path: std::path::PathBuf::from(path_in),
    };*/
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    /* open file from args:path
    let display = args.path.display();
    let _file = match std::fs::File::open(&args.path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };*/

    // read lines from file
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut i = 0;
    let mut total = 0;
    for line in content.lines() {
        if line.contains(&args.pattern) && !args.reverse {
            if !(args.first && total > 0) {
                println!("{}, {}", i, line);
            }            
            total = total + 1;
        }
        if !line.contains(&args.pattern) && args.reverse {
            println!("{}, {}", i, line);
            total = total + 1;
        }
        i = i+1;
    }
    println!("matches found: {}", total);
}