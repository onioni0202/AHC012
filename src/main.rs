use proconio::input;
use rand::prelude::*;

const SEED: u128 = 0;

struct Input {
    strawberry_num: usize,
    cut_num: usize,
    participants_num: Vec<usize>,
    strawberry_places: Vec<(i32, i32)>,
}

fn input() -> Input {
    input! {
        strawberry_num: usize,
        cut_num: usize,
        participants_num: [usize; 10],
        strawberry_places: [(i32, i32); strawberry_num]
    }
    Input {
        strawberry_num,
        cut_num,
        participants_num,
        strawberry_places,
    }
}

fn main() {
    let input_info: Input = input();
    let mut cut_edges = vec![];
    let new_cut_edges = (0, 10, 0, -10);
    let mut pieces = vec![(0..input_info.strawberry_num).collect::<Vec<_>>()];
    cut_pieces(&input_info, &mut cut_edges, &mut pieces, new_cut_edges);
    let start_time = std::time::Instant::now();
    let duration = 2.95;
    let mut rng = rand_pcg::Pcg64Mcg::new(SEED);
    while (std::time::Instant::now() - start_time).as_secs_f32() < duration
        && cut_edges.len() < input_info.cut_num
    {
        annealing(&input_info, &mut cut_edges, &mut pieces, &mut rng, 0.02);
    }
    print_solution(&cut_edges);
}

fn annealing(
    input: &Input,
    cut_edges: &mut Vec<(i32, i32, i32, i32)>,
    pieces: &mut Vec<Vec<usize>>,
    rng: &mut rand_pcg::Pcg64Mcg,
    duration: f32,
) {
    const START_TEMP: f32 = 5000.0;
    const END_TEMP: f32 = 50.0;
    let start_time = std::time::Instant::now();
    let mut solution = (rng.gen_range(-10000, 10000),
                        rng.gen_range(-10000, 10000),
                        rng.gen_range(-10000, 10000),
                        rng.gen_range(-10000, 10000));
    let start_score = compute_score(input, pieces, cut_edges[cut_edges.len()-1]);
    let mut score = start_score;
    let mut best_solution = solution.clone();
    let mut best_score = score;
    let mut iter_num = 0;

    loop {
        iter_num += 1;
        let diff_time = (std::time::Instant::now() - start_time).as_secs_f32();
        if diff_time > duration {
            break;
        }
        let select_point = rng.gen_range(0, 4);
        let diff: i32 = rng.gen_range(-1000, 1001);
        add_diff(&mut solution, select_point, diff);
        let new_score = compute_score(input, pieces, solution);
        let temp = START_TEMP + (END_TEMP - START_TEMP) * diff_time / duration;
        if new_score > best_score {
            best_score = new_score;
            best_solution = solution.clone();
        }
        if f32::exp((new_score - score) / temp) > rng.gen() {
            score = new_score;
        } else {
            add_diff(&mut solution, select_point, -diff);
        }
    }
    if start_score < best_score {
        cut_pieces(input, cut_edges, pieces, best_solution);
        eprintln!("BEST_SCORE: {}", best_score);
        eprintln!("ITER: {}", iter_num);
    }
}

fn add_diff(solution: &mut (i32, i32, i32, i32), select_point: i32, diff: i32) {
    match select_point {
        0 => {
            solution.0 += diff;
        }
        1 => {
            solution.1 += diff;
        }
        2 => {
            solution.2 += diff;
        }
        3 => {
            solution.3 += diff;
        }
        _ => unreachable!(),
    }
}

fn cut_pieces(
    input: &Input,
    cut_edges: &mut Vec<(i32, i32, i32, i32)>,
    pieces: &mut Vec<Vec<usize>>,
    new_cut_edges: (i32, i32, i32, i32),
) {
    let (px, py, qx, qy) = new_cut_edges;
    let mut new_pieces = vec![];
    for piece in pieces.clone() {
        let mut left = vec![];
        let mut right = vec![];
        for j in piece {
            let (x, y) = input.strawberry_places[j];
            let side = (qx - px) * (y - py) - (qy - py) * (x - px);
            if side > 0 {
                left.push(j);
            } else if side < 0 {
                right.push(j);
            }
        }
        if left.len() > 0 {
            new_pieces.push(left);
        }
        if right.len() > 0 {
            new_pieces.push(right);
        }
    }
    cut_edges.push(new_cut_edges);
    *pieces = new_pieces;
}

fn compute_score(
    input: &Input,
    pieces: &Vec<Vec<usize>>,
    new_cut_edges: (i32, i32, i32, i32),
) -> f32 {
    let (px, py, qx, qy) = new_cut_edges;
    let mut b = vec![0; 10];
    for piece in pieces {
        let mut left = 0;
        let mut right = 0;
        for &j in piece {
            let (x, y) = input.strawberry_places[j];
            let side = (qx - px) * (y - py) - (qy - py) * (x - px);
            if side > 0 {
                left += 1;
            } else if side < 0 {
                right += 1;
            }
        }
        if 0 < left && left <= 10 {
            b[left - 1] += 1;
        }
        if 0 < right && right <= 10 {
            b[right - 1] += 1;
        }
    }
    let mut num = 0;
    let mut den = 0;
    for d in 0..10 {
        num += input.participants_num[d].min(b[d]);
        den += input.participants_num[d];
    }
    let score = (1e6 * num as f32 / den as f32).round();
    score
}

fn print_solution(cut_edges: &Vec<(i32, i32, i32, i32)>) {
    println!("{}", cut_edges.len());
    for (px, py, qx, qy) in cut_edges {
        if (px, py) == (qx, qy) {
            println!("{} {} {} {}", px + 1, py, qx, qy);
        } else {
            println!("{} {} {} {}", px, py, qx, qy);
        }
    }
}
