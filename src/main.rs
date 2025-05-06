use constants::{ELITISM_COUNT, NUM_GENERATIONS, OUTPUT_EVERY, POPULATION_SIZE};
use evolution::{crossover, mutate, tournament_selection};
use genotype::{Genotype, Individual};
use render::{calculate_mse, render_genotype, render_target_glyph, save_buffer};
use std::fs;

mod constants;
mod evolution;
mod genotype;
mod render;
fn main() {
    let mut rng = rand::rng();
    fs::create_dir_all("results").expect("Failed to create results directory");

    let target_buffer = render_target_glyph("fonts/NotoSans-Light.ttf", 'O').unwrap();
    render::save_buffer(&target_buffer, "results/_target.png").unwrap();

    // Initialize population
    let mut population: Vec<Individual> = (0..POPULATION_SIZE)
        .map(|_| Individual::new(Genotype::new_random(&mut rng)))
        .collect();

    // Loop
    for generation in 0..NUM_GENERATIONS {
        // Evaluate fitness
        for individual in population.iter_mut() {
            let phenotype_buffer = render_genotype(&individual.genotype);
            individual.fitness = calculate_mse(&phenotype_buffer, &target_buffer);
        }

        // Sort population by fitness (lower is better)
        population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());

        // Save the best individual of this generation
        if generation % OUTPUT_EVERY == 0 || generation == NUM_GENERATIONS - 1 {
            println!(
                "Generation {}: Best fitness: {}",
                generation, population[0].fitness
            );
            let best_individual = &population[0];
            let phenotype_buffer = render_genotype(&best_individual.genotype);
            save_buffer(
                &phenotype_buffer,
                &format!("results/best_{}.png", generation),
            )
            .unwrap();
        }

        // Create new generation
        let mut new_population = Vec::with_capacity(POPULATION_SIZE);

        // Elite selection: carry over the best individuals  unchanged
        for i in 0..ELITISM_COUNT {
            if i < population.len() {
                new_population.push(population[i].clone());
            }
        }

        // Fill the rest of the new population with offspring
        while new_population.len() < POPULATION_SIZE {
            // Selection
            let parent1 = tournament_selection(&population, &mut rng);
            let parent2 = tournament_selection(&population, &mut rng);

            // Crossover
            let mut child_genotype = crossover(&parent1.genotype, &parent2.genotype, &mut rng);

            // Mutation
            mutate(&mut child_genotype, &mut rng);

            new_population.push(Individual::new(child_genotype));
        }
        population = new_population;
    }

    let final_best_individual = &population[0];
    let phenotype_buffer = render_genotype(&final_best_individual.genotype);
    save_buffer(&phenotype_buffer, "results/_final_best.png")
        .expect("Failed to save final best image");
}
