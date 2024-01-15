use std::ops::Deref;
use crate::genetic::{CrossoverType, MutationType};
use crate::graph_parser::parse_graph;

mod graph_parser;
mod graph;
mod genetic;

fn main() {
    let cmd = clap::Command::new("pea-genetic")
        .bin_name("pea-genetic")
        .version("0.0.1")
        .author("Michal Durkalec 263917@student.pwr.edu.pl")
        .about("Genetic algorithm for solving TSP problem")
        .help_template("{bin} {version} \n {about} \n {author} \n {usage} \n {all-args}")
        .arg(
            clap::Arg::new("input")
                .short('i')
                .long("input")
                .value_name("PATH")
                .help("Sets input file")
                .required(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
        )
        .arg(
            clap::Arg::new("population")
                .short('p')
                .long("population")
                .value_name("SIZE")
                .help("Sets size of population")
                .default_value("100")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            clap::Arg::new("generations")
                .short('g')
                .long("generations")
                .value_name("COUNT")
                .help("Sets number of generations")
                .value_parser(clap::value_parser!(usize))
                .default_value("100")
        )
        .arg(
            clap::Arg::new("crossover")
                .short('c')
                .long("crossover")
                .value_name("PROBABILITY")
                .help("Sets crossover probability")
                .value_parser(clap::value_parser!(f64))
                .default_value("0.8")
        )
        .arg(
            clap::Arg::new("crossover_type")
                .short('t')
                .long("crossover_type")
                .value_name("TYPE")
                .help("Sets crossover type")
                .default_value("pmx")
        )
        .arg(
            clap::Arg::new("mutation")
                .short('m')
                .long("mutation")
                .value_name("PROBABILITY")
                .help("Sets mutation probability")
                .value_parser(clap::value_parser!(f64))
                .default_value("0.01")
        )
        .arg(
            clap::Arg::new("mutation_type")
                .short('y')
                .long("mutation_type")
                .value_name("2opt | 4opt | reversion | insertion | scramble")
                .help("Sets mutation type")
                .default_value("4opt")
        );
    let matches = cmd.get_matches();
    let input = matches.get_one::<std::path::PathBuf>("input").unwrap();
    let graph = parse_graph(input).unwrap();
    let params = genetic::GAParameters {
        population_size: matches.get_one::<usize>("population").unwrap().clone(),
        crossover_probability: matches.get_one::<f64>("crossover").unwrap().clone(),
        mutation_probability: matches.get_one::<f64>("mutation").unwrap().clone(),
        mutation_type: match matches.get_one::<String>("mutation_type").unwrap().clone().deref() {
            "2opt" => MutationType::TwoOptSwap,
            "4opt" => MutationType::FourOptSwap,
            "reversion" => MutationType::Reversion,
            "insertion" => MutationType::Insertion,
            "scramble" => MutationType::Scramble,
            _ => panic!("Unknown mutation type")
        },
        max_generations: matches.get_one::<usize>("generations").unwrap().clone(),
        crossover_type: match matches.get_one::<String>("crossover_type").unwrap().clone().deref() {
            "pmx" => CrossoverType::PMX,
            "ox" => CrossoverType::OX,
            "cx" => CrossoverType::CX,
            _ => panic!("Unknown crossover type")
        }
    };
    // RUN!
    let _path = genetic::genetic_tsp(&graph, &params);
}
