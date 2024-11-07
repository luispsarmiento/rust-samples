struct Game {
    weapon:&'static str,
    power_level:u32
}

// declare a trait

trait Stats {
    fn character_stats(&self);
}

// implement a trait
impl Stats for Game {
    fn character_stats(&self){
        println!("Printing stats of power level: {}, weapon: {}", self.power_level, self.weapon);
    }
}

fn main() {
    println!("Hello, world!");

    let g1 = Game {
        power_level: 100,
        weapon: "Sword of Fire"
    };

    g1.character_stats();
}
