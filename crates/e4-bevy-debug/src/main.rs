// E4: bevy 0.20.1 default-features=false + `debug`. Expect this to compile.
use bevy::ecs::schedule::ScheduleBuildSettings;
fn main() {
    let s = ScheduleBuildSettings { shuffle_seed: Some(1), ..Default::default() };
    println!("{:?}", s.shuffle_seed);
}
