// E1: bevy_ecs 0.20.1 WITHOUT the `debug` feature. Accessing `shuffle_seed` should fail to compile.
use bevy_ecs::schedule::ScheduleBuildSettings;
fn main() {
    let s = ScheduleBuildSettings { shuffle_seed: Some(1), ..Default::default() };
    println!("{:?}", s.shuffle_seed);
}
