extern crate game_of_life;

use game_of_life::*;

fn main() {
    let mut world = init_world(80, 80);
    insert_pattern(&mut world, &get_pattern(Patterns::Toad), 10, 10);
    insert_pattern(&mut world, &get_pattern(Patterns::Blinker), 0, 0);
    insert_pattern(&mut world, &get_pattern(Patterns::Blinker), 76, 0);
    print_world(&world);
    for _i in 0..3 {
        let (lives, dies) = get_state(&world);
        change_state(&mut world, lives, dies);
        print_world(&world);
    }
}