use rand::seq::SliceRandom;
use crate::graph::{Graph, MatrixGraph};
use rand::{random, Rng, thread_rng};

pub struct GAParameters {
    pub population_size: usize,
    pub mutation_probability: f64,
    pub crossover_probability: f64,
    pub max_generations: usize,
    pub mutation_type: MutationType,
    pub crossover_type: CrossoverType,
}

pub enum MutationType {
    Swap,
    Inversion,
    Scramble,
    Insertion,
}

pub enum CrossoverType {
    PMX,
    OX,
    CX,
}

pub enum SelectionType {
    // Tournament selection, requires tournament size
    Tournament,
    // Roulette selection, requires nothing
    Roulette,
    // Stochastic Universal Sampling, requires nothing
    SUS,
}

struct SelectionCount {
    mutation: usize,
    crossover: usize,
    elitism: usize,
}

pub fn genetic_tsp(graph: &MatrixGraph, params: &GAParameters) -> Vec<usize> {
    let mut population = generate_initial_population(graph, params.population_size);
    let mut fitness: Vec<isize> = Vec::with_capacity(params.population_size);
    let mut all_time_best: isize = isize::MAX;

    for i in 0..params.max_generations {
        fitness = calculate_fitness(graph, &population);
        let mut sort_lookup = sequence_vector(params.population_size);
        sort_lookup.sort_by(|a, b| fitness[*a].cmp(&fitness[*b]));
        // Cross-over
        let crossover_pair_count = (params.crossover_probability * (params.population_size) as f64).floor() as usize;
        let children = crossover(&population, &generate_crossover_pairs(&fitness, crossover_pair_count), params);
        // New population
        let mut new_population = Vec::with_capacity(params.population_size);
        for child in &children {
            new_population.push(child.clone());
        }
        assert_eq!(new_population.len(), children.len());
        // Fill rest with best parents
        for i in 0..params.population_size - children.len() {
            new_population.push(population[sort_lookup[i]].clone());
        }
        // Mutation
        for i in 0..params.population_size {
            if random::<f64>() < params.mutation_probability {
                mutate_in_place(&mut new_population[i], params);
            }
        }
        // Overwrite old population
        population = new_population;
        // Print best fitness
        let best_fitness = fitness.iter().min().unwrap();
        if *best_fitness < all_time_best { all_time_best = *best_fitness; }
        println!("{}, {}, {}", i, best_fitness, all_time_best);
    }

    return population[0].clone();
}

fn mutate_in_place(path: &mut Vec<usize>, params: &GAParameters) {
    match params.mutation_type {
        MutationType::Swap => swap_mutate(path),
        MutationType::Inversion => inversion_mutate(path),
        MutationType::Scramble => scramble_mutate(path),
        MutationType::Insertion => insertion_mutate(path),
    }
}

fn insertion_mutate(p0: &mut Vec<usize>) {
    todo!()
}

fn scramble_mutate(p0: &mut Vec<usize>) {
    todo!()
}

fn inversion_mutate(p0: &mut Vec<usize>) {
    todo!()
}

fn swap_mutate(path: &mut Vec<usize>) {
    // Choose two random points
    let point1 = rand::thread_rng().gen_range(0..path.len());
    let point2 = rand::thread_rng().gen_range(0..path.len());
    // Swap them
    let tmp = path[point1];
    path[point1] = path[point2];
    path[point2] = tmp;
}

fn crossover(population: &Vec<Vec<usize>>, pairs: &Vec<(usize, usize)>, params: &GAParameters) -> Vec<Vec<usize>> {
    let mut children = Vec::with_capacity(population.len());
    for i in 0..pairs.len() {
        let parent1 = &population[pairs[i].0];
        let parent2 = &population[pairs[i].1];
        let child = one_point_crossover(parent1, parent2, params);
        children.push(child);
    }
    children
}

fn one_point_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, params: &GAParameters) -> Vec<usize> {
    let mut child = Vec::with_capacity(parent1.len());
    let crossover_point = rand::thread_rng().gen_range(1..parent1.len());
    for i in 0..crossover_point {
        child.push(parent1[i]);
    };
    let mut unused: Vec<usize> = Vec::with_capacity(parent1.len());
    parent2.iter().filter(|x| !child.contains(x)).for_each(|x| unused.push(*x));
    for x in unused {
        child.push(x);
    }
    child
}

fn generate_crossover_pairs(fitness: &Vec<isize>, crossover_pair_count: usize) -> Vec<(usize, usize)> {
    // TODO: Implement better parent selection - roulette, SUS
    let mut parents = sequence_vector(fitness.len());
    parents.sort_by(|a, b| fitness[*a].cmp(&fitness[*b]));
    // Generate top best pairs
    let mut pairs = Vec::with_capacity(crossover_pair_count);
    for i in 0..parents.len() {
        for j in 0..parents.len() {
            if pairs.len() == crossover_pair_count {
                return pairs;
            }
            if i != j {
                pairs.push((parents[i], parents[j]));
            }
        }
    }
    pairs
}


fn generate_initial_population(graph: &MatrixGraph, population_size: usize) -> Vec<Vec<usize>> {
    let mut population = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        population.push(generate_random_path(graph.size));
    }
    population
}

fn generate_random_path(graph_size: usize) -> Vec<usize> {
    let mut path = Vec::with_capacity(graph_size);
    for i in 0..graph_size {
        path.push(i);
    }
    path.shuffle(&mut thread_rng());
    path
}

fn calculate_fitness(graph: &MatrixGraph, population: &Vec<Vec<usize>>) -> Vec<isize> {
    let mut fitness = Vec::with_capacity(population.len());
    for i in 0..population.len() {
        fitness.push(graph.calculate_path_length(&population[i]));
    }
    fitness
}

fn sequence_vector(size: usize) -> Vec<usize> {
    let mut vec = Vec::with_capacity(size);
    for i in 0..size {
        vec.push(i);
    }
    vec
}