use std::env;
use std::process::ExitCode;

const POINTS: usize = 23;
const BLACK: usize = 3;
const COLOR: usize = 4;
const BLACK_PATTERNS: u32 = 1_771; // C(23, 3)
const COLOR_PATTERNS: u32 = 4_845; // C(20, 4)
const STATES: u32 = BLACK_PATTERNS * COLOR_PATTERNS;
const UNVISITED: u8 = u8::MAX;

type Permutation = [u8; POINTS];
type Positions3 = [usize; BLACK];
type Positions4 = [usize; COLOR];

const fn build_binomial() -> [[u32; 5]; POINTS + 1] {
    let mut binomial = [[0; 5]; POINTS + 1];
    let mut n = 0;
    while n <= POINTS {
        binomial[n][0] = 1;
        let mut k = 1;
        while k <= 4 {
            binomial[n][k] = if n == 0 {
                0
            } else {
                binomial[n - 1][k - 1] + binomial[n - 1][k]
            };
            k += 1;
        }
        n += 1;
    }
    binomial
}

const BINOMIAL: [[u32; 5]; POINTS + 1] = build_binomial();

fn rank_combination<const K: usize>(positions: &[usize; K]) -> u32 {
    positions
        .iter()
        .enumerate()
        .map(|(i, &position)| BINOMIAL[position][i + 1])
        .sum()
}

fn unrank_combination<const K: usize>(mut rank: u32, mut n: usize) -> [usize; K] {
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

fn encode(mut black: Positions3, mut color: Positions4) -> u32 {
    black.sort_unstable();
    color.sort_unstable();

    let mut compressed = [0; COLOR];
    for (i, &position) in color.iter().enumerate() {
        let preceding_black = black.partition_point(|&black_position| black_position < position);
        compressed[i] = position - preceding_black;
    }
    rank_combination(&black) * COLOR_PATTERNS + rank_combination(&compressed)
}

fn decode(id: u32) -> (Positions3, Positions4) {
    let black = unrank_combination::<BLACK>(id / COLOR_PATTERNS, POINTS);
    let compressed = unrank_combination::<COLOR>(id % COLOR_PATTERNS, POINTS - BLACK);

    let mut color = [0; COLOR];
    let mut source = 0;
    let mut target = 0;
    let mut black_index = 0;
    for position in 0..POINTS {
        if target == COLOR {
            break;
        }
        if black_index < BLACK && black[black_index] == position {
            black_index += 1;
            continue;
        }
        if compressed[target] == source {
            color[target] = position;
            target += 1;
        }
        source += 1;
    }
    (black, color)
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

fn moves(disc_turn_metric: bool) -> Vec<Permutation> {
    let mut result = Vec::with_capacity(if disc_turn_metric { 16 } else { 8 });
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

fn apply(id: u32, permutation: &Permutation) -> u32 {
    let (mut black, mut color) = decode(id);
    for position in &mut black {
        *position = permutation[*position] as usize;
    }
    for position in &mut color {
        *position = permutation[*position] as usize;
    }
    encode(black, color)
}

fn self_test() -> bool {
    for id in 0..STATES {
        let (black, color) = decode(id);
        if encode(black, color) != id {
            return false;
        }
    }

    let samples = [
        0,
        1,
        4_844,
        4_845,
        123_456,
        4_000_000,
        8_580_494,
        STATES - 1,
    ];
    for disc_turn_metric in [false, true] {
        let metric_moves = moves(disc_turn_metric);
        if metric_moves.len() != if disc_turn_metric { 16 } else { 8 } {
            return false;
        }
        for id in samples {
            if metric_moves
                .iter()
                .any(|movement| apply(id, movement) >= STATES)
            {
                return false;
            }
            if !disc_turn_metric {
                for generator in 0..4 {
                    let moved = apply(id, &metric_moves[2 * generator]);
                    if apply(moved, &metric_moves[2 * generator + 1]) != id {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn print_positions(black: &Positions3, color: &Positions4) {
    print!("  farthest black positions:");
    for position in black {
        print!(" {}", position + 1);
    }
    print!("\n  farthest tracked-color positions:");
    for position in color {
        print!(" {}", position + 1);
    }
    println!();
}

fn analyze(disc_turn_metric: bool) {
    let metric_moves = moves(disc_turn_metric);
    let mut distance = vec![UNVISITED; STATES as usize];
    let mut queue = vec![0_u32; STATES as usize];
    let mut tail = 0_usize;

    let solved_black = [0, 1, 2];
    for column in 0..5 {
        let solved_color = [3 + column, 8 + column, 13 + column, 18 + column];
        let goal = encode(solved_black, solved_color);
        if distance[goal as usize] == UNVISITED {
            distance[goal as usize] = 0;
            queue[tail] = goal;
            tail += 1;
        }
    }

    let mut histogram = [0_u64; 64];
    let mut head = 0_usize;
    let mut radius = 0_u8;
    let mut farthest = queue[0];
    while head < tail {
        let current = queue[head];
        head += 1;
        let current_distance = distance[current as usize];
        histogram[current_distance as usize] += 1;
        if current_distance > radius {
            radius = current_distance;
            farthest = current;
        }
        for movement in &metric_moves {
            let next = apply(current, movement);
            if distance[next as usize] == UNVISITED {
                distance[next as usize] = current_distance + 1;
                queue[tail] = next;
                tail += 1;
            }
        }
    }

    let (black, color) = decode(farthest);
    println!(
        "{} metric",
        if disc_turn_metric {
            "Disc-turn"
        } else {
            "Fifth-turn"
        }
    );
    println!("  moves: {}", metric_moves.len());
    println!("  abstract states reached: {tail} / {STATES}");
    println!("  exact black-plus-one-color goal radius: {radius}");
    print!("  distance histogram:");
    for depth in 0..=radius {
        print!(" {depth}:{}", histogram[depth as usize]);
    }
    println!();
    print_positions(&black, &color);
}

fn run() -> Result<(), &'static str> {
    if !self_test() {
        return Err("self-test failed");
    }

    let arguments: Vec<_> = env::args_os().skip(1).collect();
    if arguments.len() == 1 && arguments[0] == "--self-test" {
        println!("optimal-number self-test passed");
        return Ok(());
    }
    if !arguments.is_empty() {
        return Err("usage: optimal-number [--self-test]");
    }

    analyze(false);
    analyze(true);
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
