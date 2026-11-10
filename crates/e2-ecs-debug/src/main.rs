//! E2: bevy_ecs 0.20.1 + `debug`. SingleThreadedExecutor explicitly. One process = one seed.
//! Usage: e2-ecs-debug <none|u64>
//! Prints one line per scenario:
//!   A: graph isomorphic to official test `schedule_builds_randomly_with_shuffler`
//!   B: hidden read/write dependency (produce writes Value, consume reads it), NO ordering
//!   C: explicit constraints: chain(c1,c2,c3), x.before(y), z.after(y) + free systems f1..f4
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{ScheduleBuildSettings, SingleThreadedExecutor};

#[derive(Resource, Default)]
struct Log(Vec<&'static str>);
#[derive(Resource, Default)]
struct Counters(Vec<u32>);
#[derive(Resource, Default)]
struct Value(u32);
#[derive(Resource, Default)]
struct Observed(Option<u32>);

fn settings(seed: Option<u64>) -> ScheduleBuildSettings {
    ScheduleBuildSettings { shuffle_seed: seed, ..Default::default() }
}

fn new_schedule(seed: Option<u64>) -> Schedule {
    let mut s = Schedule::default();
    s.set_executor(SingleThreadedExecutor::new());
    s.set_build_settings(settings(seed));
    s
}

// ---------- A ----------
fn sysn<const N: u32>(mut c: ResMut<Counters>) { c.0.push(N); }
fn scenario_a(seed: Option<u64>) -> Vec<u32> {
    let mut schedule = new_schedule(seed);
    schedule.add_systems(sysn::<0>);
    schedule.add_systems((sysn::<10>, sysn::<11>).after(sysn::<0>));
    schedule.add_systems((sysn::<20>, sysn::<21>).after(sysn::<10>));
    let mut world = World::new();
    world.init_resource::<Counters>();
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world);
    world.remove_resource::<Counters>().unwrap().0
}

// ---------- B ----------
fn produce(mut v: ResMut<Value>, mut log: ResMut<Log>) { v.0 = 42; log.0.push("produce"); }
fn consume(v: Res<Value>, mut o: ResMut<Observed>, mut log: ResMut<Log>) { o.0 = Some(v.0); log.0.push("consume"); }
fn noise1(mut log: ResMut<Log>) { log.0.push("noise1"); }
fn noise2(mut log: ResMut<Log>) { log.0.push("noise2"); }
fn scenario_b(seed: Option<u64>) -> (Vec<&'static str>, Option<u32>) {
    let mut schedule = new_schedule(seed);
    // deliberately NO before/after between produce and consume
    schedule.add_systems((consume, noise1, produce, noise2));
    let mut world = World::new();
    world.init_resource::<Log>();
    world.init_resource::<Value>();
    world.init_resource::<Observed>();
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world);
    (world.remove_resource::<Log>().unwrap().0, world.resource::<Observed>().0)
}

// ---------- C ----------
macro_rules! named { ($($n:ident),*) => { $( fn $n(mut log: ResMut<Log>) { log.0.push(stringify!($n)); } )* } }
named!(c1, c2, c3, x, y, z, f1, f2, f3, f4);
fn scenario_c(seed: Option<u64>) -> (Vec<&'static str>, bool) {
    let mut schedule = new_schedule(seed);
    schedule.add_systems((f1, f2));
    schedule.add_systems((c1, c2, c3).chain());
    schedule.add_systems(x.before(y));
    schedule.add_systems(y);
    schedule.add_systems(z.after(y));
    schedule.add_systems((f3, f4));
    let mut world = World::new();
    world.init_resource::<Log>();
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world);
    let order = world.remove_resource::<Log>().unwrap().0;
    let pos = |n: &str| order.iter().position(|s| *s == n).expect("system ran");
    let ok = pos("c1") < pos("c2") && pos("c2") < pos("c3") && pos("x") < pos("y") && pos("y") < pos("z");
    (order, ok)
}

fn main() {
    let arg = std::env::args().nth(1).expect("usage: <none|u64>");
    let seed = if arg == "none" { None } else { Some(arg.parse::<u64>().expect("u64")) };
    let s = arg.as_str();
    println!("A seed={s} order={:?}", scenario_a(seed));
    let (ob, val) = scenario_b(seed);
    println!("B seed={s} order={:?} consume_saw={:?}", ob, val);
    let (oc, ok) = scenario_c(seed);
    println!("C seed={s} order={:?} constraints_ok={ok}", oc);
}
