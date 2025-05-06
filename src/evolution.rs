use rand::Rng;

use crate::{
    constants::{
        MAX_COORD, MUTATION_AMOUNT, MUTATION_RATE, NUM_LINES, POINT_MUTATION_RATE, TOURNAMENT_SIZE,
    },
    genotype::{Genotype, Individual},
};

pub fn tournament_selection<'a>(
    population: &'a [Individual],
    rng: &mut impl Rng,
) -> &'a Individual {
    let mut best_participant: Option<&'a Individual> = None;
    for _ in 0..TOURNAMENT_SIZE {
        let idx = rng.random_range(0..population.len());
        let participant = &population[idx];
        if best_participant.is_none() || participant.fitness < best_participant.unwrap().fitness {
            best_participant = Some(participant);
        }
    }
    best_participant.unwrap()
}

pub fn crossover(parent1: &Genotype, parent2: &Genotype, rng: &mut impl Rng) -> Genotype {
    assert_eq!(parent1.lines.len(), NUM_LINES);
    assert_eq!(parent2.lines.len(), NUM_LINES);

    let crossover_point = rng.random_range(1..NUM_LINES);
    let mut child_lines = Vec::with_capacity(NUM_LINES);
    child_lines.extend_from_slice(&parent1.lines[0..crossover_point]);
    child_lines.extend_from_slice(&parent2.lines[crossover_point..NUM_LINES]);
    Genotype { lines: child_lines }
}

pub fn mutate(genotype: &mut Genotype, rng: &mut impl Rng) {
    for line_idx in 0..genotype.lines.len() {
        if rng.random_bool(MUTATION_RATE) {
            if rng.random_bool(POINT_MUTATION_RATE) {
                let old_x = genotype.lines[line_idx].start.x;
                let old_y = genotype.lines[line_idx].start.y;
                genotype.lines[line_idx].start.x = mutate_coord(old_x, rng);
                genotype.lines[line_idx].start.y = mutate_coord(old_y, rng);
            }
            if rng.random_bool(POINT_MUTATION_RATE) {
                let old_x = genotype.lines[line_idx].end.x;
                let old_y = genotype.lines[line_idx].end.y;
                genotype.lines[line_idx].end.x = mutate_coord(old_x, rng);
                genotype.lines[line_idx].end.y = mutate_coord(old_y, rng);
            }
        }
    }
}

fn mutate_coord(v: i32, rng: &mut impl Rng) -> i32 {
    (v + rng.random_range(-MUTATION_AMOUNT..MUTATION_AMOUNT)).clamp(0, MAX_COORD)
}
