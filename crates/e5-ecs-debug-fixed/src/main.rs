//! E5: bevy_ecs 0.20.1 + `debug`. SingleThreadedExecutor explicitly. One process = one seed.
//! Follow-up to E2 scenario B: same systems, two schedules per process.
//!   B: hidden dependency, NO ordering (identical to E2 scenario B, re-run as control)
//!   F: same systems, constraint added: `consume.after(produce)` (the fix shown in the draft, step 5)
//! Usage: e5-ecs-debug-fixed <none|u64>
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{ScheduleBuildSettings, SingleThreadedExecutor};

#[derive(Resource, Default)]
struct Log(Vec<&'static str>);
#[derive(Resource, Default)]
struct Value(u32);
#[derive(Resource, Default)]
struct Observed(Option<u32>);

fn new_schedule(seed: Option<u64>) -> Schedule {
    let mut s = Schedule::default();
    s.set_executor(SingleThreadedExecutor::new());
    s.set_build_settings(ScheduleBuildSettings { shuffle_seed: seed, ..Default::default() });
    s
}

fn produce(mut v: ResMut<Value>, mut log: ResMut<Log>) { v.0 = 42; log.0.push("produce"); }
fn consume(v: Res<Value>, mut o: ResMut<Observed>, mut log: ResMut<Log>) { o.0 = Some(v.0); log.0.push("consume"); }
fn noise1(mut log: ResMut<Log>) { log.0.push("noise1"); }
fn noise2(mut log: ResMut<Log>) { log.0.push("noise2"); }

fn run(mut schedule: Schedule) -> (Vec<&'static str>, Option<u32>) {
    let mut world = World::new();
    world.init_resource::<Log>();
    world.init_resource::<Value>();
    world.init_resource::<Observed>();
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world);
    (world.remove_resource::<Log>().unwrap().0, world.resource::<Observed>().0)
}

fn scenario_b(seed: Option<u64>) -> (Vec<&'static str>, Option<u32>) {
    let mut schedule = new_schedule(seed);
    // deliberately NO before/after between produce and consume
    schedule.add_systems((consume, noise1, produce, noise2));
    run(schedule)
}

fn scenario_f(seed: Option<u64>) -> (Vec<&'static str>, Option<u32>) {
    let mut schedule = new_schedule(seed);
    // the fix: consume runs after produce
    schedule.add_systems((produce, consume.after(produce), noise1, noise2));
    run(schedule)
}

fn main() {
    let arg = std::env::args().nth(1).expect("usage: <none|u64>");
    let seed = if arg == "none" { None } else { Some(arg.parse::<u64>().expect("u64")) };
    let (ob, vb) = scenario_b(seed);
    println!("B seed={arg} order={ob:?} consume_saw={vb:?}");
    let (of, vf) = scenario_f(seed);
    println!("F seed={arg} order={of:?} consume_saw={vf:?}");
}
