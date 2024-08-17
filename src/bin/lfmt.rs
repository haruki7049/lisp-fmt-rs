use clap::Parser;

fn main() {
    let args: CommandLineArgs = CommandLineArgs::parse();

    if args.verbose {
        eprintln!("This verbose option is not implemented yet...");
        todo!();
    }
    if args.inplace {
        eprintln!("This inplace option is not implemented yet...");
        todo!();
    }
    if !args.style.is_empty() {
        eprintln!("This style option is not implemented yet...");
        todo!();
    }
    if args.length != 0 {
        eprintln!("This length option is not implemented yet...");
        todo!();
    }
    if args.lines != 0 {
        eprintln!("This lines option is not implemented yet...");
        todo!();
    }
    if args.offset != 0 {
        eprintln!("This offset option is not implemented yet...");
        todo!();
    }
    if args.output_replacements_xml {
        eprintln!("This output_replacements_xml option is not implemented yet...");
        todo!();
    }
    if args.sort_includes {
        eprintln!("This sort_includes option is not implemented yet...");
        todo!();
    }
    if args.cursor != 0 {
        eprintln!("This cursor option is not implemented yet...");
        todo!();
    }
    if args.dump_config {
        eprintln!("This dump_config option is not implemented yet...");
        todo!();
    }

    println!("The file argument's value: {}", args.file);
    todo!();
}

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct CommandLineArgs {
    file: String,

    #[arg(long, default_value_t = false)]
    verbose: bool,

    #[arg(long, default_value_t = false)]
    inplace: bool,

    #[arg(long, default_value_t = 0)]
    length: usize,

    #[arg(long, default_value_t = 0)]
    lines: usize,

    #[arg(long, default_value_t = 0)]
    offset: usize,

    #[arg(long, default_value_t = String::from("{}"))]
    style: String,

    #[arg(long, default_value_t = false)]
    output_replacements_xml: bool,
    #[arg(long, default_value_t = false)]
    sort_includes: bool,
    #[arg(long, default_value_t = 0)]
    cursor: usize,
    #[arg(long, default_value_t = false)]
    dump_config: bool,
}
