// E3: bevy 0.20.1 default-features=false, WITHOUT `debug`. Expect compile failure on `shuffle_seed`.
use bevy::ecs::schedule::ScheduleBuildSettings;
fn main() {
    let s = ScheduleBuildSettings { shuffle_seed: Some(1), ..Default::default() };
    println!("{:?}", s.shuffle_seed);
}
