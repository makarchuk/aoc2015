use std::collections::HashMap;
use std::io::{self, Read};

struct Graph<'a> {
    nodes: Vec<&'a str>,
    routes: HashMap<(&'a str, &'a str), u16>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        return Graph {
            nodes: vec![],
            routes: HashMap::new(),
        };
    }

    fn get_price(&self, from: &'a str, to: &'a str) -> u16 {
        self.routes[&(from, to)]
    }

    fn the_most_expensive_way(&self) -> u16 {
        Self::permutations(&self.nodes)
            .iter()
            .map(|permutation| {
                let mut last_node = "";
                permutation.iter().fold(0, |total_price, node| {
                    if last_node != "" {
                        let price = self.get_price(last_node, node);
                        last_node = node;
                        total_price + price
                    } else {
                        last_node = node;
                        total_price
                    }
                })
            }).max()
            .ok_or("No prices found!")
            .unwrap()
    }

    fn cheapest_way(&self) -> u16 {
        Self::permutations(&self.nodes)
            .iter()
            .map(|permutation| {
                let mut last_node = "";
                permutation.iter().fold(0, |total_price, node| {
                    if last_node != "" {
                        let price = self.get_price(last_node, node);
                        last_node = node;
                        total_price + price
                    } else {
                        last_node = node;
                        total_price
                    }
                })
            }).min()
            .ok_or("No prices found!")
            .unwrap()
    }

    fn permutations(collection: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
        if collection.len() == 1 {
            return vec![vec![collection[0]]];
        }
        let mut result = vec![];
        for el in collection {
            for tail in
                Self::permutations(&collection.iter().filter(|x| *x != el).map(|x| *x).collect())
            {
                let mut whole = vec![*el];
                whole.extend(tail);
                result.push(whole.clone())
            }
        }
        return result;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut graph = Graph::new();
    for line in input.split("\n") {
        let chunks: Vec<&str> = line.split(" ").collect();
        let from = chunks[0];
        let to = chunks[2];
        let cost = chunks[4].parse().unwrap();
        if !graph.nodes.contains(&from) {
            graph.nodes.push(from.clone());
        }
        if !graph.nodes.contains(&to) {
            graph.nodes.push(to.clone());
        }
        graph.routes.insert((from.clone(), to.clone()), cost);
        graph.routes.insert((to, from), cost);
    }
    println!("Cheapest way is {}", graph.cheapest_way());
    println!(
        "The most expensive way is {}",
        graph.the_most_expensive_way()
    );
}
