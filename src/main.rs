use structopt::StructOpt;

mod cli;
mod data;
mod generate;
mod find;
mod load;

fn main() {
    let opt = cli::Opt::from_args();
    println!("{:?}", opt);

    match opt.command {
        cli::Command::Generate { number, length, min, max, file_name} => {
            println!("Generating {} vectors, {} of size, with range {} to {} and writing to file: {} ", number, length, min, max, file_name );
            let vectors = generate::generate_vectors(&number, &length, &min, &max);
            let _ = generate::write_vectors(&file_name, &vectors);
            println!("Success!")
        },
        cli::Command::Find { k_nearest, implementation, target_file, search_space_file} => {
            println!("Finding {} k_nearest vectors for randomly generated vector using  impl: {}",
                     &k_nearest, implementation);


            let target_vecs = load::load_vectors(&target_file);
            let search_space = load::load_vectors(&search_space_file);

            match target_vecs {
                Ok(target) => {
                    match search_space {
                        Ok(sp) => {
                            for t in target {
                                let result = find::k_nearest(&t, &sp, &k_nearest, &implementation);
                                println!("Result for {:?} is {:?}", t, result)
                            }
                        },
                        Err(_) => panic!("failed to read search space file")
                    }
                },
                Err(_) => panic!("failed to read target vector file")
            }

        },
    }
}
