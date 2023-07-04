use structopt::StructOpt;
use crate::data::NnImpls;

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