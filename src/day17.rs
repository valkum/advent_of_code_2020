use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
#[cfg(test)]
use itertools::Itertools;
use legion::{
    systems::CommandBuffer,
    world::{Duplicate, SubWorld},
    *,
};
use rayon::iter::ParallelIterator;
#[cfg(test)]
use std::collections::HashMap;
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct X(i64);
#[derive(Clone, Copy, Debug, PartialEq)]
struct Y(i64);
#[derive(Clone, Copy, Debug, PartialEq)]
struct Z(i64);
#[derive(Clone, Copy, Debug, PartialEq)]
struct W(i64);
#[derive(Clone, Copy, Debug, PartialEq)]
struct PosOffset {
    x: i64,
    y: i64,
    z: i64,
    w: Option<i64>,
}
impl PosOffset {
    fn new(x: i64, y: i64, z: i64) -> Self {
        PosOffset { x, y, z, w: None }
    }
    fn new4(x: i64, y: i64, z: i64, w: i64) -> Self {
        PosOffset {
            x,
            y,
            z,
            w: Some(w),
        }
    }
}

impl Add<PosOffset> for (X, Y, Z) {
    type Output = (X, Y, Z);

    fn add(self, rhs: PosOffset) -> Self::Output {
        (
            X(self.0 .0 + rhs.x),
            Y(self.1 .0 + rhs.y),
            Z(self.2 .0 + rhs.z),
        )
    }
}
impl Add<PosOffset> for (X, Y, Z, W) {
    type Output = (X, Y, Z, W);

    fn add(self, rhs: PosOffset) -> Self::Output {
        (
            X(self.0 .0 + rhs.x),
            Y(self.1 .0 + rhs.y),
            Z(self.2 .0 + rhs.z),
            W(self.3 .0 + rhs.w.unwrap()),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Active;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> World {
    let mut world = World::default();
    let z = 0;
    let mut y = 0;
    let mut x;
    for line in input.lines().rev() {
        x = 0;
        for line_char in line.chars() {
            match line_char {
                '#' => world.push((X(x), Y(y), Z(z), W(0), Active)),
                '.' => world.push((X(x), Y(y), Z(z), W(0))),
                _ => unimplemented!(),
            };
            x += 1;
        }
        y += 1;
    }
    // for x0 in -1..=x+1 {
    //     for y0 in -1..=y {
    //         for z0 in -1..=1 {
    //             if !((0..=x).contains(&x0) && (0..=y).contains(&y0) && (0..=z).contains(&z0)) {
    //                 world.push((X(x0), Y(y0), Z(z0), Active(false)));
    //             }
    //         }
    //     }
    // }

    world
}

// fn set_active(commands: &mut CommandBuffer, entity: Entity, active: bool) {
//     commands.exec_mut(move |w| {
//         if let Some(mut entry) = w.entry(entity) {
//             // access the entity's components, returns `None` if the entity does not have the component
//             entry.get_component_mut::<Active>().unwrap().0 = active;
//         }
//     })
// }
#[system]
#[read_component(Entity)]
#[read_component(X)]
#[read_component(Y)]
#[read_component(Z)]
#[read_component(Active)]
fn update_active(world: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] kernel: &Kernel) {
    let mut active_query = <(Entity, &X, &Y, &Z)>::query().filter(component::<Active>());
    let mut inactive_query = <(Entity, &X, &Y, &Z)>::query().filter(!component::<Active>());

    let active_col: Vec<(Entity, (X, Y, Z))> = active_query
        .par_iter(world)
        .map(|(e, x, y, z)| (*e, (*x, *y, *z)))
        .collect();
    let inactive_col: Vec<(Entity, (X, Y, Z))> = inactive_query
        .par_iter(world)
        .map(|(e, x, y, z)| (*e, (*x, *y, *z)))
        .collect();

    for (e, pos) in active_col.iter() {
        match active_query
            .par_iter(world)
            .filter(|(_, x, y, z)| kernel.iter().any(|offset| *pos + offset == (**x, **y, **z)))
            .count()
        {
            2 | 3 => {}
            _ => {
                cmd.remove_component::<Active>(*e)
                // set_active(cmd, *e, false)
            }
        }
    }
    for (e, pos) in inactive_col.iter() {
        match active_query
            .par_iter(world)
            .filter(|(_, x, y, z)| kernel.iter().any(|offset| *pos + offset == (**x, **y, **z)))
            .count()
        {
            3 => {
                cmd.add_component(*e, Active)
                // set_active(cmd, *e, true)
            }
            _ => {}
        }
    }
}
#[system]
#[read_component(Entity)]
#[read_component(X)]
#[read_component(Y)]
#[read_component(Z)]
#[read_component(W)]
#[read_component(Active)]
fn update_active_4d(world: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] kernel: &Kernel) {
    let mut active_query = <(Entity, &X, &Y, &Z, &W)>::query().filter(component::<Active>());
    let mut inactive_query = <(Entity, &X, &Y, &Z, &W)>::query().filter(!component::<Active>());

    let active_col: Vec<(Entity, (X, Y, Z, W))> = active_query
        .par_iter(world)
        .map(|(e, x, y, z, w)| (*e, (*x, *y, *z, *w)))
        .collect();
    let inactive_col: Vec<(Entity, (X, Y, Z, W))> = inactive_query
        .par_iter(world)
        .map(|(e, x, y, z, w)| (*e, (*x, *y, *z, *w)))
        .collect();

    for (e, pos) in active_col.iter() {
        match active_query
            .par_iter(world)
            .filter(|(_, x, y, z, w)| {
                kernel
                    .iter()
                    .any(|offset| *pos + offset == (**x, **y, **z, **w))
            })
            .count()
        {
            2 | 3 => {}
            _ => cmd.remove_component::<Active>(*e),
        }
    }
    for (e, pos) in inactive_col.iter() {
        match active_query
            .par_iter(world)
            .filter(|(_, x, y, z, w)| {
                kernel
                    .iter()
                    .any(|offset| *pos + offset == (**x, **y, **z, **w))
            })
            .count()
        {
            3 => cmd.add_component(*e, Active),
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct WorldState {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
    w: (i64, i64),
}
impl Default for WorldState {
    fn default() -> Self {
        WorldState {
            x: (0, 2),
            y: (0, 2),
            z: (0, 0),
            w: (0, 0),
        }
    }
}
fn remove_unused(commands: &mut CommandBuffer, e: Entity) {
    commands.exec_mut(move |world| {
        world.remove(e);
    });
}

#[system]
#[read_component(Entity)]
#[read_component(X)]
#[read_component(Y)]
#[read_component(Z)]
#[read_component(Active)]
fn expand_world(world: &mut SubWorld, #[state] state: &mut WorldState, cmd: &mut CommandBuffer) {
    let mut query = <(&X, &Y, &Z)>::query().filter(component::<Active>());
    let mut bb = WorldState {
        x: (0, 0),
        y: (0, 0),
        z: (0, 0),
        w: (0, 0),
    };

    query.iter(world).for_each(|(x, y, z)| {
        bb.x.0 = bb.x.0.min(x.0);
        bb.x.1 = bb.x.1.max(x.0);
        bb.y.0 = bb.y.0.min(y.0);
        bb.y.1 = bb.y.1.max(y.0);
        bb.z.0 = bb.z.0.min(z.0);
        bb.z.1 = bb.z.1.max(z.0);
        // println!("({},{},{}) is active", x.0,y.0,z.0);
    });
    let new_state = bb.clone();
    for x in new_state.x.0 - 1..=new_state.x.1 + 1 {
        for y in new_state.y.0 - 1..=new_state.y.1 + 1 {
            for z in new_state.z.0 - 1..=new_state.z.1 + 1 {
                if !(bb.x.0..=bb.x.1).contains(&x)
                    || !(bb.y.0..=bb.y.1).contains(&y)
                    || !(bb.z.0..=bb.z.1).contains(&z)
                {
                    cmd.push((X(x), Y(y), Z(z)));
                }
            }
        }
    }
    for x in state.x.0 - 1..=state.x.1 + 1 {
        for y in state.y.0 - 1..=state.y.1 + 1 {
            for z in state.z.0 - 1..=state.z.1 + 1 {
                if !(new_state.x.0..=new_state.x.1).contains(&x)
                    || !(new_state.y.0..=new_state.y.1).contains(&y)
                    || !(new_state.z.0..=new_state.z.1).contains(&z)
                {
                    let e = <(Entity, &X, &Y, &Z)>::query()
                        .par_iter(world)
                        .find_any(|(_, xe, ye, ze)| xe.0 == x && ye.0 == y && ze.0 == z)
                        .map(|(e, _, _, _)| *e);
                    if let Some(e) = e {
                        remove_unused(cmd, e);
                    }
                }
            }
        }
    }

    *state = new_state;
}

#[system]
#[read_component(Entity)]
#[read_component(X)]
#[read_component(Y)]
#[read_component(Z)]
#[read_component(W)]
#[read_component(Active)]
fn expand_world_4d(world: &mut SubWorld, #[state] state: &mut WorldState, cmd: &mut CommandBuffer) {
    let mut query = <(&X, &Y, &Z, &W)>::query().filter(component::<Active>());
    let mut bb = WorldState {
        x: (0, 0),
        y: (0, 0),
        z: (0, 0),
        w: (0, 0),
    };

    query.iter(world).for_each(|(x, y, z, w)| {
        bb.x.0 = bb.x.0.min(x.0);
        bb.x.1 = bb.x.1.max(x.0);
        bb.y.0 = bb.y.0.min(y.0);
        bb.y.1 = bb.y.1.max(y.0);
        bb.z.0 = bb.z.0.min(z.0);
        bb.z.1 = bb.z.1.max(z.0);
        bb.w.0 = bb.w.0.min(w.0);
        bb.w.1 = bb.w.1.max(w.0);
        // println!("({},{},{}) is active", x.0,y.0,z.0);
    });
    let new_state = bb.clone();
    for x in new_state.x.0 - 1..=new_state.x.1 + 1 {
        for y in new_state.y.0 - 1..=new_state.y.1 + 1 {
            for z in new_state.z.0 - 1..=new_state.z.1 + 1 {
                for w in new_state.w.0 - 1..=new_state.w.1 + 1 {
                    if !(bb.x.0..=bb.x.1).contains(&x)
                        || !(bb.y.0..=bb.y.1).contains(&y)
                        || !(bb.z.0..=bb.z.1).contains(&z)
                        || !(bb.w.0..=bb.w.1).contains(&w)
                    {
                        cmd.push((X(x), Y(y), Z(z), W(w)));
                    }
                }
            }
        }
    }
    for x in state.x.0 - 1..=state.x.1 + 1 {
        for y in state.y.0 - 1..=state.y.1 + 1 {
            for z in state.z.0 - 1..=state.z.1 + 1 {
                for w in new_state.w.0 - 1..=new_state.w.1 + 1 {
                    if !(new_state.x.0..=new_state.x.1).contains(&x)
                        || !(new_state.y.0..=new_state.y.1).contains(&y)
                        || !(new_state.z.0..=new_state.z.1).contains(&z)
                    {
                        let e = <(Entity, &X, &Y, &Z, &W)>::query()
                            .par_iter(world)
                            .find_any(|(_, xe, ye, ze, we)| {
                                xe.0 == x && ye.0 == y && ze.0 == z && we.0 == w
                            })
                            .map(|(e, _, _, _, _)| *e);
                        if let Some(e) = e {
                            remove_unused(cmd, e);
                        }
                    }
                }
            }
        }
    }

    *state = new_state;
}

#[cfg(test)]
fn pretty_print(world: &World) {
    println!("Num Entities: {}", world.len());
    let mut printmap = HashMap::new();
    for (x, y, z) in <(&X, &Y, &Z)>::query()
        .filter(component::<Active>())
        .iter(world)
    {
        let entry = printmap.entry(z.0).or_insert(HashMap::new());
        let entry = entry.entry(y.0).or_insert(HashMap::new());
        entry.entry(x.0).or_insert("#");
    }
    for (x, y, z) in <(&X, &Y, &Z)>::query()
        .filter(!component::<Active>())
        .iter(world)
    {
        let entry = printmap.entry(z.0).or_insert(HashMap::new());
        let entry = entry.entry(y.0).or_insert(HashMap::new());
        entry.entry(x.0).or_insert("#");
    }
    for (i, a) in printmap.iter().sorted_by_key(|x| x.0) {
        println!("Z={}", i);

        for (i, s) in a.iter().sorted_by_key(|x| -x.0) {
            print!("{:3}: ", i);
            for (_, t) in s.iter().sorted_by_key(|x| x.0) {
                print!("{}", t);
            }
            println!();
        }
    }
}
#[cfg(not(test))]
fn pretty_print(_: &World) {}

#[derive(PartialEq, Debug)]
enum Part {
    One,
    Two,
}
pub struct Kernel(Part);
impl Kernel {
    fn iter(&self) -> Box<dyn Iterator<Item = PosOffset> + '_> {
        match &self.0 {
            Part::Two => Box::new(
                (-1..=1)
                    .map(|x| {
                        (-1..=1)
                            .map(move |y| {
                                (-1..=1)
                                    .map(move |z| {
                                        (-1..=1).filter_map(move |w| {
                                            if !(x == 0 && y == 0 && z == 0 && w == 0) {
                                                Some(PosOffset::new4(x, y, z, w))
                                            } else {
                                                None
                                            }
                                        })
                                    })
                                    .flatten()
                            })
                            .flatten()
                    })
                    .flatten(),
            ),
            Part::One => Box::new(
                (-1..=1)
                    .map(|x| {
                        (-1..=1)
                            .map(move |y| {
                                (-1..=1).filter_map(move |z| {
                                    if !(x == 0 && y == 0 && z == 0) {
                                        Some(PosOffset::new(x, y, z))
                                    } else {
                                        None
                                    }
                                })
                            })
                            .flatten()
                    })
                    .flatten(),
            ),
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &World) -> usize {
    let mut merger = Duplicate::default();
    merger.register_copy::<X>();
    merger.register_copy::<Y>();
    merger.register_copy::<Z>();
    merger.register_copy::<Active>();
    let mut world = World::default();
    world.clone_from(input, &any(), &mut merger);
    let mut schedule = Schedule::builder()
        .add_system(expand_world_system(WorldState::default()))
        .flush()
        .add_system(update_active_system())
        .build();

    let mut resources = Resources::default();
    resources.insert(Kernel(Part::One));
    resources.insert(Part::One);

    pretty_print(&world);
    for i in 1..=6 {
        schedule.execute(&mut world, &mut resources);
        println!("Step {}:", i);
        pretty_print(&world);
    }
    let mut query = <&Active>::query();

    return query.iter(&world).count();
}

#[aoc(day17, part2)]
pub fn part2(input: &World) -> usize {
    let mut merger = Duplicate::default();
    merger.register_copy::<X>();
    merger.register_copy::<Y>();
    merger.register_copy::<Z>();
    merger.register_copy::<W>();
    merger.register_copy::<Active>();
    let mut world = World::default();
    world.clone_from(input, &any(), &mut merger);
    let mut schedule = Schedule::builder()
        .add_system(expand_world_4d_system(WorldState::default()))
        .flush()
        .add_system(update_active_4d_system())
        .build();

    let mut resources = Resources::default();
    resources.insert(Kernel(Part::Two));
    resources.insert(Part::Two);

    pretty_print(&world);
    for i in 1..=6 {
        schedule.execute(&mut world, &mut resources);
        println!("Step {}:", i);
        pretty_print(&world);
    }
    let mut query = <&Active>::query();

    return query.iter(&world).count();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = ".#.
..#
###";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE1)), 112);
    }
    #[test]
    fn sample_kernel() {
        assert_eq!(Kernel(Part::One).iter().count(), 26);
        assert_eq!(Kernel(Part::Two).iter().count(), 80);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE1)), 848);
    }
}
