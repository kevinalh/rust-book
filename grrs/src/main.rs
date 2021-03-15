use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::from_args();
    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.to_str().expect("Could not read file"));
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
