use structopt::StructOpt;
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub enum NnImpls {
    Heap,
    Parallel,
    Sort,
    KDTree,
    LocalitySensitiveHashing,
    HierarchicalSmallWord
}

//TODO revisit this to understand better, plus add other types
impl FromStr for NnImpls {
    type Err = &'static str; // Change this to be the type of error you want to return

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Heap" | "heap" | "h" => Ok(NnImpls::Heap),
            // Add other matches for other variants
            _ => Err("no match"),
        }
    }
}

impl fmt::Display for NnImpls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NnImpls::Heap => write!(f, "Heap"),
            _ => write!(f, "not yet supported approach")
            // Add other matches for other variants
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum Command {

    #[structopt(name = "generate", about = "generate vectors")]
     Generate{
       #[structopt(short, long)]
        number: usize,
        #[structopt(short, long, default_value="100")]
        length: usize,
        #[structopt(long, default_value="0.0")]
        min: f32,
        #[structopt(long, default_value="1.0")]
        max: f32,
        #[structopt(short, long, default_value="vectors.json")]
        file_name: String
    },
    #[structopt(name = "find", about = "find k nearest vectors")]
    Find {
        #[structopt(short, long)]
        k_nearest: usize,
        #[structopt(short, long)]
        implementation: NnImpls,
        #[structopt(short, long, default_value="target_vec.json")]
        target_file: String,
        #[structopt(short, long, default_value="vectors.json")]
        search_space_file: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "vector_perf", about = "Analyzing nearest vector performance")]
pub struct Opt {

    #[structopt(subcommand)]
    pub command: Command

}