pub enum Patterns {
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    GosperGliderGun,
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

        Patterns::GosperGliderGun => vec![
            //-> 1       2      3      4      5      6      7      8      9      10     11     12     13     14     15     16     17     18     19     20     21     22     23     24     25     26     27     28     29     30     31     32     33     34     35     36
       /*1*/vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true , false, false, false, false, false, false, false, false, false, false, false],
       /*2*/vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true , false, true , false, false, false, false, false, false, false, false, false, false, false],
       /*3*/vec![false, false, false, false, false, false, false, false, false, false, false, false, true , true , false, false, false, false, false, false, true , true , false, false, false, false, false, false, false, false, false, false, false, false, true , true ],
       /*4*/vec![false, false, false, false, false, false, false, false, false, false, false, true , false, false, false, true , false, false, false, false, true , true , false, false, false, false, false, false, false, false, false, false, false, false, true , true ],
       /*5*/vec![true , true , false, false, false, false, false, false, false, false, true , false, false, false, false, false, true , false, false, false, true , true , false, false, false, false, false, false, false, false, false, false, false, false, false, false],
       /*6*/vec![true , true , false, false, false, false, false, false, false, false, true , false, false, false, true , false, true , true , false, false, false, false, true , false, true , false, false, false, false, false, false, false, false, false, false, false],
       /*7*/vec![false, false, false, false, false, false, false, false, false, false, true , false, false, false, false, false, true , false, false, false, false, false, false, false, true , false, false, false, false, false, false, false, false, false, false, false],
       /*8*/vec![false, false, false, false, false, false, false, false, false, false, false, true , false, false, false, true , false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
       /*9*/vec![false, false, false, false, false, false, false, false, false, false, false, false, true , true , false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        ],
    }
}

pub fn rotate(v: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let (x_ubound, _) = get_bounds(v);
    let mut v_p = Vec::new();

    for &(x, y) in v.iter() {
        let x_p = y;
        let y_p = x_ubound - x;
        v_p.push((x_p, y_p));
    }

    v_p
}

pub fn get_bounds(v: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut x_ubound = 0;
    let mut y_ubound = 0;

    for &(x, y) in v.iter() {
        if x > x_ubound {
            x_ubound = x;
        }

        if y > y_ubound {
            y_ubound = y;
        }
    }

    (x_ubound, y_ubound)
}

pub fn matricize(v: &Vec<bool>) {
    //todo
}

pub fn print_vec(v: &Vec<(usize, usize)>) {
    let (x_ubound, y_ubound) = get_bounds(&v);
    for y in 0..y_ubound+1 {
        for x in 0..x_ubound+1 {
            if v.contains(&(x, y)) {
                print!("1");
            } else {
                print!("0");
            }
        }
        print!("\n");
    }
}
// 0,1,0
// 1,1,1
// 0,1,0
// 0,1,0
// 0,1,0


#[test]
fn test_vec_code() {
    let v = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2), (1, 3), (1, 4)];
    print_vec(&v);
    let v_p = rotate(&v);
    print!("\n");
    print_vec(&v_p);
}