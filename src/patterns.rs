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