use indextree::{Arena, NodeId};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
struct Orbit {
    name: String,
    orbited_by: BTreeSet<Orbit>,
}

type Tree = BTreeMap<String, BTreeSet<String>>;

fn parse_into(input: &str) -> (Arena<String>, BTreeMap<String, NodeId>) {
    let connections: &mut Arena<String> = &mut Arena::new();
    let mut ids_by_name: BTreeMap<String, NodeId> = BTreeMap::new();

    for raw in input.lines() {
        let split = raw.split(')').collect::<Vec<_>>();
        match split[..] {
            [orbitee, orbited_by] => {
                let mut get_or_set = |name: &str| {
                    ids_by_name.get(name).map(|&n| n).unwrap_or_else(|| {
                        let new_id = connections.new_node(name.to_string());
                        ids_by_name.insert(name.to_string(), new_id);
                        new_id
                    })
                };

                let id_1 = get_or_set(orbitee);
                let id_2 = get_or_set(orbited_by);

                id_1.append(id_2, connections);
            }
            _ => panic!("Shouldn't happen."),
        }
    }

    (connections.clone(), ids_by_name)
}

fn part_1(input: Arena<String>) -> usize {
    input
        .iter()
        .map(|n| {
            let mut i = 0;
            let mut curr = n.parent();

            while let Some(c) = curr {
                curr = input.get(c).and_then(|n| n.parent());
                i += 1
            }

            i
        })
        .sum()
}

fn part_2(input: Arena<String>, node_ids: BTreeMap<String, NodeId>) -> usize {
    let you = node_ids["YOU"];
    let santa = node_ids["SAN"];

    let get_transfers = |id: NodeId| {
        let mut transfers = BTreeMap::new();

        let mut i = 0;
        let mut curr = input.get(id).unwrap().parent();

        while let Some(c) = curr {
            transfers.insert(i, c);

            curr = input.get(c).and_then(|n| n.parent());
            i += 1;
        }
        transfers
    };

    let you_transfers = get_transfers(you);
    let santa_transfers = get_transfers(santa);

    for (i, k1) in you_transfers {
        for (j, k2) in santa_transfers.clone() {
            if k1 == k2 {
                println!("{} {} {} {}", i, j, k1, k2);
                return i + j;
            }
        }
    }

    panic!("Shouldn't get here!")
}

pub fn challenge() -> Result<(), std::io::Error> {
    let mut file = File::open("./data/day-6")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Day 6");

    let (arena, node_ids) = parse_into(&contents);

    println!("Part 1 {:?}", part_1(arena.clone()));
    println!("Part 2 {:?}", part_2(arena, node_ids));

    Ok(())
}
