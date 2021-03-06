extern crate rand;
use rand::{thread_rng, Rng};

fn get_score(event: [u8; 144]) -> f32 {
    let mut game: [bool; 6 * 24] = [false; 6 * 24];
    let mut opponents: [bool; 24 * 24] = [false; 24 * 24];
    for r in 0..6 {
        let r: usize = r * 24;
        for table in 0..6 {
            let t: usize = table * 4 + r;
            for player in 0..4 {
                let player: usize = event[t + player] as usize;
                game[player * 6 + table] = true;
                for p in 0..4 {
                    let p: usize = event[t + p] as usize;
                    opponents[player * 24 + p] = true;
                }
            }
        }
    }
    let mut score: f32 = 0.0;
    for i in game.iter() {
        if *i {
            score += 1.0;
        }
    }
    let mut min: f32 = 19.0;
    let mut total: f32 = 0.0;
    for p in 0..24 {
        let mut now: f32 = 0.0;
        for o in 0..24 {
            if opponents[p * 24 + o] {
                now += 1.0;
            }
        }
        if now < min {
            min = now;
        }
        total += now;
    }
    score += min;
    score += total / 24.0;
    score
}

fn gen_layout() -> [u8; 144] {
    let mut rng = thread_rng();
    let mut event: [u8; 144] = [0; 144];
    let mut options: [u8; 24] = [0;24];
    for i in 0..24 {
        event[i as usize] = i;
        options[i as usize] = i;
    }
    for r in 1..6 {
        let r: usize = r * 24;
        rng.shuffle(&mut options);
        for pos in 0..24 {
            event[r + pos as usize] = options[pos as usize];
        }
    }
    event
}

fn main() {
    println!("Welcome to Social Scheduler");
    let mut event: [u8; 144] = gen_layout();
    let mut cevent: [u8; 144];
    let mut score: f32 = get_score(event);
    let mut max: f32 = score;
    let mut iterations: u32 = 0;
    loop {
        iterations += 1;
        let mut changed: bool = false;
        let mut new: [u8; 144] = [0; 144];
        for r in 0..6 {
            let r: usize = r * 24;
            for t1 in 0..6 {
                let t1: usize = r + t1 * 4;
                for t2 in 0..6 {
                    let t2: usize = r + t2 * 4;
                    if t1 != t2 {
                        for p1 in 0..4 {
                            for p2 in 0..4 {
                                cevent = event;
                                cevent.swap(t1 + p1, t2 + p2);
                                let cscore: f32 = get_score(cevent);
                                if cscore > score {
                                    score = cscore;
                                    new = cevent;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        if changed {
            event = new;
        } else {
            if score >= max {
                println!(
                    "Found local max: {:?} after {:?} iterations ({:?} attempts)",
                    score,
                    iterations,
                    6 * 6 * 5 * 4 * 4 * iterations,
                );
                max = score;
                let mut json_event: Vec<Vec<Vec<u8>>> = Vec::with_capacity(6);
                for r in 0..6 {
                    let mut round: Vec<Vec<u8>> = Vec::with_capacity(6);
                    for t in 0..6 {
                        let mut table: Vec<u8> = Vec::with_capacity(4);
                        for p in 0..4 {
                            table.push(event[r * 24 + t * 4 + p]);
                        }
                        round.push(table);
                    }
                    json_event.push(round);
                }
                println!("{:?}", json_event);
            }
            iterations = 0;
            event = gen_layout();
            score = get_score(event);
        }
    }
}
