pub enum Patterns {
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    // Pentadecathlon,
}

pub fn get_pattern(pattern: Patterns) -> Vec<Vec<bool>> {
    match pattern {
        Patterns::Blinker => vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ],

        Patterns::Toad => vec![
            vec![false, false, false, false],
            vec![false, true, true, true],
            vec![true, true, true, false],
            vec![false, false, false, false],
        ],

        Patterns::Beacon => vec![
            vec![true, true, false, false],
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![false, false, true, true],
        ],

        Patterns::Pulsar => vec![
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, true , true , true , false, false, false, true , true , true , false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, true , false, false, false, false, true , false, true , false, false, false, false, true , false],
            vec![false, true , false, false, false, false, true , false, true , false, false, false, false, true , false],
            vec![false, true , false, false, false, false, true , false, true , false, false, false, false, true , false],
            vec![false, false, false, true , true , true , false, false, false, true , true , true , false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, true , true , true , false, false, false, true , true , true , false, false, false],
            vec![false, true , false, false, false, false, true , false, true , false, false, false, false, true , false],
            vec![false, true , false, false, false, false, true , false, true , false, false, false, false, true , false],
            vec![false, true , false, false, false, false, true , false, true , false, false, false, false, true , false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, true , true , true , false, false, false, true , true , true , false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        ],
    }
}

pub fn init_world(x: usize, y: usize) -> Vec<Vec<bool>> {
    vec![vec![false; x]; y]
}

pub fn insert_pattern(world: &mut Vec<Vec<bool>>, pattern: &Vec<Vec<bool>>, x: usize, y: usize) {
    for i in 0..pattern.len() {
        for j in 0..pattern[i].len() {
            world[y + i][x + j] = pattern[i][j];
        }
    }
}

pub fn get_num_alive_neighbors(world: &Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let mut count: u32 = 0;
    // y-1, x-1
    match y.checked_sub(1) {
        Some(y_n) => match x.checked_sub(1) {
            Some(x_n) => {
                if world[y_n][x_n] {
                    count = count + 1;
                }
            }
            None => {}
        },
        None => {}
    }

    //y-1, x
    match y.checked_sub(1) {
        Some(y_n) => {
            if world[y_n][x] {
                count = count + 1;
            }
        }
        None => {}
    }

    //y-1, x+1
    match y.checked_sub(1) {
        Some(y_n) => {
            if x + 1 < world[y_n].len() {
                if world[y_n][x + 1] {
                    count = count + 1;
                }
            }
        }
        None => {}
    }

    //y, x-1
    match x.checked_sub(1) {
        Some(x_n) => {
            if world[y][x_n] {
                count = count + 1;
            }
        }
        None => {}
    }

    //y, x+1
    if x + 1 < world[y].len() {
        if world[y][x + 1] {
            count = count + 1;
        }
    }

    //y+1, x-1
    if y + 1 < world.len() {
        match x.checked_sub(1) {
            Some(x_n) => {
                if world[y + 1][x_n] {
                    count = count + 1;
                }
            }
            None => {}
        }
    }

    //y+1, x
    if y + 1 < world.len() {
        if world[y + 1][x] {
            count = count + 1;
        }
    }

    //y+1, x+1
    if y + 1 < world.len() {
        if x + 1 < world[y + 1].len() {
            if world[y + 1][x + 1] {
                count = count + 1;
            }
        }
    }

    count
}

// 1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
// 2. Any live cell with two or three live neighbors lives on to the next generation.
// 3. Any live cell with more than three live neighbors dies, as if by overpopulation.
// 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
pub fn get_state(world: &Vec<Vec<bool>>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut lives = Vec::new();
    let mut dies = Vec::new();
    for y in 0..world.len() {
        for x in 0..world[0].len() {
            let num_neighbors = get_num_alive_neighbors(&world, x, y);
            if world[y][x] {
                //rule 1
                if num_neighbors < 2 {
                    dies.push((x, y));
                }
                //rule 2
                if num_neighbors == 2 || num_neighbors == 3 {
                    lives.push((x, y));
                }
                //rule 3
                if num_neighbors > 3 {
                    dies.push((x, y));
                }
            } else {
                if num_neighbors == 3 {
                    lives.push((x, y));
                }
            }
        }
    }
    (lives, dies)
}

pub fn change_state(world: &mut Vec<Vec<bool>>, lives: Vec<(usize, usize)>, dies: Vec<(usize, usize)>) {
    for coords in lives.iter() {
        let (x, y) = *coords;
        world[y][x] = true;
    }

    for coords in dies.iter() {
        let (x, y) = *coords;
        world[y][x] = false;
    }
}

pub fn print_world(world: &Vec<Vec<bool>>) {
    for y in 0..world.len() {
        for x in 0..world[0].len() {
            if world[y][x] {
                print!("*");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}