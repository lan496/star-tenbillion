use std::collections::{HashMap, HashSet};
use std::env;
use std::process::ExitCode;
use std::thread;

const POINTS: usize = 23;
const OTHERS: usize = 20;
const STATES: u64 = 4_509_264_634_875; // 23! / (3! (4!)^5 5!)
const PARTITIONS: u64 = 2_546_168_625; // 20! / ((4!)^5 5!)
const BLOCK_CHOICES: [u64; 4] = [969, 455, 165, 35]; // C(19,3), C(15,3), C(11,3), C(7,3)
const THREADS: usize = 4;
const CANDIDATES_PER_PASS: u64 = 60_000_000;
const MASK_BITS: u32 = 16;
const MASK: u64 = (1 << MASK_BITS) - 1;
const MAX_CONTINUATION: usize = 24;
const FIFTH_FACTOR: usize = 10;
const DISC_FACTOR: usize = 7;

type Permutation = [u8; POINTS];
// Color 0 is the three-ball color; the four-ball colors carry arbitrary labels.
type Coloring = [u8; POINTS];

const fn build_binomial() -> [[u64; POINTS + 1]; POINTS + 1] {
    let mut binomial = [[0; POINTS + 1]; POINTS + 1];
    let mut n = 0;
    while n <= POINTS {
        binomial[n][0] = 1;
        let mut k = 1;
        while k <= n {
            binomial[n][k] = binomial[n - 1][k - 1] + binomial[n - 1][k];
            k += 1;
        }
        n += 1;
    }
    binomial
}

const BINOMIAL: [[u64; POINTS + 1]; POINTS + 1] = build_binomial();

fn unrank_combination<const K: usize>(mut rank: u64, mut n: usize) -> [usize; K] {
    let mut positions = [0; K];
    for i in (1..=K).rev() {
        let mut x = n - 1;
        while BINOMIAL[x][i] > rank {
            x -= 1;
        }
        positions[i - 1] = x;
        rank -= BINOMIAL[x][i];
        n = x;
    }
    positions
}

// Rank a coloring modulo the names of the five four-ball colors: the black
// positions, then each block as the three later members of its first unused
// position.
fn rank(coloring: &Coloring) -> u64 {
    let mut black_rank = 0;
    let mut black_seen = 0;
    let mut others = [0; OTHERS];
    let mut n = 0;
    for (position, &color) in coloring.iter().enumerate() {
        if color == 0 {
            black_seen += 1;
            black_rank += BINOMIAL[position][black_seen];
        } else {
            others[n] = color;
            n += 1;
        }
    }

    let mut used = 0_u32;
    let mut partition_rank = 0;
    let mut block = 0;
    for first in 0..OTHERS {
        if block == BLOCK_CHOICES.len() {
            break;
        }
        if used >> first & 1 == 1 {
            continue;
        }
        used |= 1 << first;
        let mut index = 0;
        let mut found = 0;
        let mut block_rank = 0;
        for later in first + 1..OTHERS {
            if used >> later & 1 == 1 {
                continue;
            }
            if others[later] == others[first] {
                found += 1;
                block_rank += BINOMIAL[index][found];
                used |= 1 << later;
            }
            index += 1;
        }
        partition_rank = partition_rank * BLOCK_CHOICES[block] + block_rank;
        block += 1;
    }
    black_rank * PARTITIONS + partition_rank
}

fn unrank(id: u64) -> Coloring {
    let black = unrank_combination::<3>(id / PARTITIONS, POINTS);
    let mut partition_rank = id % PARTITIONS;
    let mut block_ranks = [0; 4];
    for block in (0..4).rev() {
        block_ranks[block] = partition_rank % BLOCK_CHOICES[block];
        partition_rank /= BLOCK_CHOICES[block];
    }

    let mut labels = [5_u8; OTHERS];
    let mut free = [0_usize; OTHERS];
    for (i, slot) in free.iter_mut().enumerate() {
        *slot = i;
    }
    let mut free_len = OTHERS;
    for (block, &block_rank) in block_ranks.iter().enumerate() {
        let later = unrank_combination::<3>(block_rank, free_len - 1);
        let label = block as u8 + 1;
        labels[free[0]] = label;
        for &i in &later {
            labels[free[i + 1]] = label;
        }
        let mut kept = 0;
        for i in 1..free_len {
            if !later.contains(&(i - 1)) {
                free[kept] = free[i];
                kept += 1;
            }
        }
        free_len = kept;
    }

    let mut coloring = [0; POINTS];
    let mut next_label = 0;
    for (position, color) in coloring.iter_mut().enumerate() {
        if !black.contains(&position) {
            *color = labels[next_label];
            next_label += 1;
        }
    }
    coloring
}

fn identity() -> Permutation {
    std::array::from_fn(|position| position as u8)
}

fn add_cycle(permutation: &mut Permutation, cycle: [usize; 5]) {
    for i in 0..5 {
        permutation[cycle[i]] = cycle[(i + 1) % 5] as u8;
    }
}

fn compose(first: &Permutation, second: &Permutation) -> Permutation {
    std::array::from_fn(|position| second[first[position] as usize])
}

// T, B, t, b as in src/optimal_number.rs.
fn generators() -> [Permutation; 4] {
    let mut result = [identity(); 4];
    add_cycle(&mut result[0], [3, 4, 5, 6, 7]);
    add_cycle(&mut result[0], [8, 9, 10, 11, 12]);
    add_cycle(&mut result[1], [13, 14, 15, 16, 17]);
    add_cycle(&mut result[1], [18, 19, 20, 21, 22]);
    add_cycle(&mut result[2], [0, 4, 1, 2, 7]);
    add_cycle(&mut result[2], [3, 9, 5, 6, 12]);
    add_cycle(&mut result[3], [8, 14, 10, 11, 17]);
    add_cycle(&mut result[3], [13, 19, 15, 16, 22]);
    result
}

// Fifth-turn moves are g, g^-1 per generator; disc-turn moves are g^1..g^4.
fn moves(disc_turn_metric: bool) -> Vec<Permutation> {
    let mut result = Vec::new();
    for generator in generators() {
        let mut power = generator;
        for exponent in 1..=4 {
            if disc_turn_metric || exponent == 1 || exponent == 4 {
                result.push(power);
            }
            power = compose(&power, &generator);
        }
    }
    result
}

fn apply(coloring: &Coloring, permutation: &Permutation) -> Coloring {
    let mut result = [0; POINTS];
    for (position, &color) in coloring.iter().enumerate() {
        result[permutation[position] as usize] = color;
    }
    result
}

fn solved() -> Coloring {
    std::array::from_fn(|position| {
        if position < 3 {
            0
        } else {
            ((position - 3) % 5) as u8 + 1
        }
    })
}

// Shortlex-least words, comparing moves by index.  Take, for each state, the
// least of its shortest move sequences.  It is also the least word for its
// element of the move group, and so is every factor of it.  Listing the least
// words up to a fixed length by breadth-first search on the group therefore
// bounds the number of states at each distance.
fn word_key(word: &[u8]) -> u64 {
    word.iter().fold(word.len() as u64, |key, &movement| key << 4 | movement as u64)
}

fn least_words(metric_moves: &[Permutation], length: usize) -> HashSet<u64> {
    let mut seen = HashSet::from([identity()]);
    let mut least = HashSet::from([word_key(&[])]);
    let mut level = vec![(Vec::new(), identity())];
    for _ in 0..length {
        let mut next = Vec::new();
        for (word, element) in &level {
            for (movement, permutation) in metric_moves.iter().enumerate() {
                let product = compose(element, permutation);
                if seen.insert(product) {
                    let mut longer = word.clone();
                    longer.push(movement as u8);
                    least.insert(word_key(&longer));
                    next.push((longer, product));
                }
            }
        }
        level = next;
    }
    least
}

// continuations[j][m]: words of length j starting with move m whose factors of
// length `factor` are all least words.  The automaton state is the last
// factor - 1 moves.
fn continuations(least: &HashSet<u64>, factor: usize, move_count: usize) -> Vec<Vec<u128>> {
    let mut states: Vec<Vec<u8>> = vec![Vec::new()];
    let mut index = HashMap::from([(word_key(&[]), 0_usize)]);
    let mut transitions: Vec<Vec<Option<usize>>> = Vec::new();
    let mut i = 0;
    while i < states.len() {
        let mut row = Vec::with_capacity(move_count);
        for movement in 0..move_count as u8 {
            let mut word = states[i].clone();
            word.push(movement);
            if !least.contains(&word_key(&word)) {
                row.push(None);
                continue;
            }
            if word.len() == factor {
                word.remove(0);
            }
            let next = *index.entry(word_key(&word)).or_insert_with(|| {
                states.push(word);
                states.len() - 1
            });
            row.push(Some(next));
        }
        transitions.push(row);
        i += 1;
    }

    // words[s]: words of the current length that the automaton accepts from s.
    let mut words = vec![1_u128; states.len()];
    let mut result = vec![vec![0; move_count]];
    for _ in 1..=MAX_CONTINUATION {
        result.push(transitions[0].iter().map(|next| next.map_or(0, |s| words[s])).collect());
        words = transitions
            .iter()
            .map(|row| row.iter().flatten().map(|&s| words[s]).sum())
            .collect();
    }
    result
}

struct Ball {
    sizes: Vec<u64>,
    // Histogram of the last level by the set of moves that end a shortest path.
    last_moves: HashMap<u64, u64>,
}

fn contains(level: &[u64], id: u64) -> bool {
    level.binary_search_by_key(&id, |entry| entry >> MASK_BITS).is_ok()
}

// Exact breadth-first search.  Each entry packs a state id with the set of
// moves that reach it from the previous level.  The last level is only
// histogrammed, never stored.
fn breadth_first(disc_turn_metric: bool, depth: usize) -> Ball {
    let metric_moves = moves(disc_turn_metric);
    let mut sizes = vec![1];
    let mut last_moves = HashMap::new();
    let mut previous: Vec<u64> = Vec::new();
    let mut current = vec![rank(&solved()) << MASK_BITS];
    if depth == 0 {
        last_moves.insert(0, 1);
    }
    for distance in 1..=depth {
        let passes = current.len() as u64 * metric_moves.len() as u64 / CANDIDATES_PER_PASS + 1;
        let span = STATES / passes + 1;
        let mut next = Vec::new();
        let mut size = 0;
        for pass in 0..passes {
            let (low, high) = (pass * span, (pass + 1) * span);
            let chunk = current.len().div_ceil(THREADS);
            let mut found: Vec<u64> = thread::scope(|scope| {
                let workers: Vec<_> = current
                    .chunks(chunk)
                    .map(|entries| {
                        let (metric_moves, previous, current) = (&metric_moves, &previous, &current);
                        scope.spawn(move || {
                            let mut found = Vec::new();
                            for &entry in entries {
                                let coloring = unrank(entry >> MASK_BITS);
                                for (movement, permutation) in metric_moves.iter().enumerate() {
                                    let id = rank(&apply(&coloring, permutation));
                                    if (low..high).contains(&id)
                                        && !contains(previous, id)
                                        && !contains(current, id)
                                    {
                                        found.push(id << MASK_BITS | 1 << movement);
                                    }
                                }
                            }
                            found
                        })
                    })
                    .collect();
                workers.into_iter().flat_map(|worker| worker.join().unwrap()).collect()
            });
            found.sort_unstable();
            let mut merged: Vec<u64> = Vec::with_capacity(found.len() / 2);
            for entry in found {
                match merged.last_mut() {
                    Some(last) if *last >> MASK_BITS == entry >> MASK_BITS => *last |= entry & MASK,
                    _ => merged.push(entry),
                }
            }
            size += merged.len() as u64;
            if distance == depth {
                for entry in &merged {
                    *last_moves.entry(entry & MASK).or_insert(0) += 1;
                }
            } else {
                next.extend(merged);
            }
        }
        sizes.push(size);
        previous = std::mem::take(&mut current);
        current = next;
    }
    Ball { sizes, last_moves }
}

// Every state at distance depth + j has a least shortest path whose first
// depth moves end at a last-level state x.  Its next move forms a least
// two-move word with some move ending a shortest path to x, and the remaining
// moves are a word counted by continuations.  Return the resulting bounds on the number of states within
// depth + j, stopping at the first one that reaches STATES.
fn counting_bounds(disc_turn_metric: bool, ball: &Ball) -> Vec<u128> {
    let metric_moves = moves(disc_turn_metric);
    let move_count = metric_moves.len();
    let factor = if disc_turn_metric { DISC_FACTOR } else { FIFTH_FACTOR };
    let least = least_words(&metric_moves, factor);
    let table = continuations(&least, factor, move_count);
    let mut within = ball.sizes.iter().map(|&n| n as u128).sum::<u128>();
    let mut bounds = Vec::new();
    for length in 1..=MAX_CONTINUATION {
        for (&last, &count) in &ball.last_moves {
            let followers = (0..move_count).filter(|&next| {
                last == 0
                    || (0..move_count).any(|previous| {
                        last >> previous & 1 == 1 && least.contains(&word_key(&[previous as u8, next as u8]))
                    })
            });
            within += count as u128 * followers.map(|next| table[length][next]).sum::<u128>();
        }
        bounds.push(within);
        if within >= STATES as u128 {
            break;
        }
    }
    bounds
}

fn self_test() -> bool {
    let samples = [0, 1, 2_546_168_624, 2_546_168_625, 123_456_789_012, STATES - 1, rank(&solved())];
    for id in samples {
        if rank(&unrank(id)) != id {
            return false;
        }
    }
    // Relabelling the four-ball colors must not change the rank.
    let solved_state = solved();
    let relabelled = solved_state.map(|color| if color == 0 { 0 } else { 6 - color });
    if rank(&relabelled) != rank(&solved_state) {
        return false;
    }
    // Small exact levels, cross-checked by an independent dictionary search.
    let expected = [1, 6, 30, 144, 692, 3_322];
    breadth_first(false, 5).sizes == expected
}

fn run() -> Result<(), &'static str> {
    if !self_test() {
        return Err("self-test failed");
    }
    let usage = "usage: full-ball --self-test | full-ball (fifth|disc) <depth>";
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.len() == 1 && arguments[0] == "--self-test" {
        println!("full-ball self-test passed");
        return Ok(());
    }
    if arguments.len() != 2 {
        return Err(usage);
    }
    let disc_turn_metric = match arguments[0].as_str() {
        "fifth" => false,
        "disc" => true,
        _ => return Err(usage),
    };
    let depth: usize = arguments[1].parse().map_err(|_| usage)?;

    let ball = breadth_first(disc_turn_metric, depth);
    println!("{} metric", if disc_turn_metric { "Disc-turn" } else { "Fifth-turn" });
    let mut within = 0;
    for (distance, &size) in ball.sizes.iter().enumerate() {
        within += size;
        println!("  distance {distance:2}: {size:>15}  within: {within:>15}");
    }
    let bounds = counting_bounds(disc_turn_metric, &ball);
    for (j, &bound) in bounds.iter().enumerate() {
        println!("  states within {:2} <= {bound:>17}", depth + j + 1);
    }
    println!("  God's number >= {}", depth + bounds.len());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
