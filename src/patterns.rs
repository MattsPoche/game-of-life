pub enum Patterns {
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    Gosper_Glider_Gun,
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

        Patterns::Gosper_Glider_Gun => vec![
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