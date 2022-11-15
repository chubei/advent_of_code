use std::collections::{hash_map::Entry, HashMap};

const INPUT: &str = "re-js
qx-CG
start-js
start-bj
qx-ak
js-bj
ak-re
CG-ak
js-CG
bj-re
ak-lg
lg-CG
qx-re
WP-ak
WP-end
re-lg
end-ak
WP-re
bj-CG
qx-start
bj-WP
JG-lg
end-lg
lg-iw";

#[derive(Debug, Clone, Copy, PartialEq)]
enum CaveType {
    Small,
    Big,
}

#[derive(Debug, Clone, Default)]
struct Graph {
    nodes: HashMap<String, (usize, CaveType)>,
}

impl Graph {
    fn get_node(&mut self, name: &str) -> (usize, CaveType) {
        let index = self.nodes.len();
        match self.nodes.entry(name.to_string()) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let cave_type = if name == name.to_ascii_lowercase() {
                    CaveType::Small
                } else {
                    CaveType::Big
                };
                let node = (index, cave_type);
                v.insert(node);
                node
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<usize>,
    visited: Vec<bool>,
    small_cave_visited_twice: bool,
}

impl Path {
    fn new(num_nodes: usize, initial_node: usize) -> Self {
        let mut visited = vec![false; num_nodes];
        visited[initial_node] = true;
        Self {
            path: vec![initial_node],
            visited,
            small_cave_visited_twice: false,
        }
    }

    fn expand(
        self,
        edges: &[Vec<(usize, CaveType)>],
        start_node: usize,
        end_node: usize,
    ) -> impl Iterator<Item = Path> + '_ {
        let node = self.path.last().unwrap();
        let visited = self.visited.clone();
        let small_cave_visited_twice = self.small_cave_visited_twice;
        edges[*node]
            .iter()
            .filter(move |(n, cave_type)| {
                Self::can_visit(
                    *n,
                    *cave_type,
                    &visited,
                    start_node,
                    end_node,
                    small_cave_visited_twice,
                )
            })
            .map(move |(n, cave_type)| {
                let mut visited = self.visited.clone();
                let mut small_cave_visited_twice = self.small_cave_visited_twice;
                if visited[*n] && *cave_type == CaveType::Small {
                    small_cave_visited_twice = true;
                }
                visited[*n] = true;
                let mut path = self.path.clone();
                path.push(*n);
                Path {
                    path,
                    visited,
                    small_cave_visited_twice,
                }
            })
    }

    fn can_visit(
        node: usize,
        cave_type: CaveType,
        visited: &[bool],
        start_node: usize,
        end_node: usize,
        small_cave_visited_twice: bool,
    ) -> bool {
        match cave_type {
            CaveType::Small => {
                if node == start_node || node == end_node {
                    !visited[node]
                } else {
                    !visited[node] || !small_cave_visited_twice
                }
            }
            CaveType::Big => true,
        }
    }
}

fn main() {
    let mut graph = Graph::default();
    let mut edges = vec![];
    for line in INPUT.lines() {
        let mut parts = line.split('-');
        let from = graph.get_node(parts.next().unwrap());
        let to = graph.get_node(parts.next().unwrap());

        add_edge(from.0, to, &mut edges);
        add_edge(to.0, from, &mut edges);
    }

    let start_node = graph.get_node("start").0;
    let end_node = graph.get_node("end").0;

    let mut final_paths = vec![];
    let mut paths = vec![Path::new(edges.len(), start_node)];
    while !paths.is_empty() {
        let mut new_paths = vec![];
        for path in paths {
            for path in path.expand(&edges, start_node, end_node) {
                if path.path.last().unwrap() == &end_node {
                    final_paths.push(path);
                } else {
                    new_paths.push(path);
                }
            }
        }
        paths = new_paths;
    }

    println!("{}", final_paths.len());
}

fn add_edge(from: usize, to: (usize, CaveType), edges: &mut Vec<Vec<(usize, CaveType)>>) {
    if edges.len() <= from {
        edges.resize(from + 1, vec![]);
    }
    edges[from].push(to);
}
