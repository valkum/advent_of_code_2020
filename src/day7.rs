use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use petgraph::algo::*;
use petgraph::data::FromElements;
use petgraph::prelude::*;
use petgraph::visit::Reversed;
use petgraph::visit::Walker;
use std::cell::RefCell;
use std::collections::HashSet;
use string_interner::{DefaultSymbol, StringInterner};

use regex::Regex;
thread_local! {
    static BAGS: RefCell<StringInterner> = Default::default();
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> DiGraphMap<DefaultSymbol, u8> {
    let mut dag = DiGraphMap::new();
    let re = Regex::new(r"(\d{1,2}) ([a-z]* [a-z]*) bags?").unwrap();
    for line in input.lines() {
        let rule = line.split("contain").collect::<Vec<&str>>();
        let root_bag_tmp = rule[0].replace("bags", "");
        let root_bag = BAGS.with(|f| f.borrow_mut().get_or_intern(root_bag_tmp.as_str().trim()));

        for right in re.captures_iter(rule[1]) {
            // let left_node = if dag.node_indices().find(|i| dag[*i] == *root_bag).is_none() {
            //      dag.add_node(root_bag.to_owned())
            // } else {
            //     dag.node_indices().find(|i| dag[*i] == *root_bag).unwrap()
            // };
            let right_bag = BAGS.with(|f| f.borrow_mut().get_or_intern(&right[2]));
            let amount = &right[1];
            // let right_node = if dag.node_indices().find(|i| dag[*i] == *right_bag).is_none() {
            //     dag.add_node(right_bag.to_owned())
            // } else {
            //     dag.node_indices().find(|i| dag[*i] == *right_bag).unwrap()
            // };

            dag.add_edge(root_bag, right_bag, amount.parse::<u8>().unwrap());
        }
    }
    return dag;
}

#[aoc(day7, part1)]
pub fn part1(input: &DiGraphMap<DefaultSymbol, u8>) -> usize {
    let graph = Reversed(input);
    let gold_node = BAGS.with(|f| f.borrow_mut().get_or_intern("shiny gold"));
    Dfs::new(graph, gold_node).iter(graph).count() - 1
}

#[aoc(day7, part2)]
pub fn part2(input: &DiGraphMap<DefaultSymbol, u8>) -> u32 {
    let gold_node = BAGS.with(|f| f.borrow_mut().get_or_intern("shiny gold"));
    // Dfs::new(input, gold_node)
    //     .iter(input)
    //     .fold(0u32, |acc, x| acc + (input.edges(x).map(|(_,_, ew)| *ew as u32).product::<u32>()))
    // let mut dfs = DfsPostOrder::new(input, gold_node);
    // let mut bags = 1u32;
    // // while let Some(node) = dfs.next(input) {
    // //     let mut edges = input.edges(node);
    // //     while let Some(edge) = edges.next() {
    // //         bags *= *edge.2 as u32
    // //     }
    // // }

    // Thanks Wanja!
    fn transitive_children(graph: &DiGraphMap<DefaultSymbol, u8>, node: DefaultSymbol) -> u32 {
        graph
            .edges(node)
            .map(|(_, next, n)| (*n as u32) * (1 + transitive_children(graph, next)))
            .sum::<u32>()
    }
    // assert_eq!(transitive_children(input, gold_node), bags);
    transitive_children(input, gold_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const SAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    #[test]
    fn sample1() {
        let input = input_generator(&SAMPLE);
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn sample2() {
        let input = input_generator(&SAMPLE2);
        assert_eq!(part2(&input), 126);
    }
}
