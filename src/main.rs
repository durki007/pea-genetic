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
        .arg(
            clap::Arg::new("input")
                .short('i')
                .long("input")
                .value_name("PATH")
                .help("Sets input file")
                .required(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
        );
    let matches = cmd.get_matches();
    let input = matches.get_one::<std::path::PathBuf>("input").unwrap();
    let graph = parse_graph(input).unwrap();
    let params = genetic::GAParameters {
        population_size: 1000,
        crossover_probability: 0.6,
        mutation_probability: 0.01,
        mutation_type: MutationType::Swap,
        max_generations: 100,
        crossover_type: CrossoverType::PMX,
    };
    println!("Hello, world! {:?}", input.as_path());
    // RUN!
    let path = genetic::genetic_tsp(&graph, &params);
}
