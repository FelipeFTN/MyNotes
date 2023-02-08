mod display;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "MyNotes", about = "A Daily Note Reminder")]
struct Opt {
    #[structopt(short = "n", long = "new", help = "Create a new note")]
    new: Option<String>,
}

fn main() {
    display::help();
    
    let opt = Opt::from_args();
    println!("{:?}", opt.new);
}
