use std::collections::HashSet;

use daggy::{
    petgraph::{
        visit::{EdgeRef, IntoEdgesDirected, IntoNodeIdentifiers, Bfs},
        Direction,
    },
    Dag, NodeIndex, Walker,
};

#[derive(Debug, Clone, Copy)]
struct Segment {
    first: usize,
    last: usize,
}

impl Segment {
    fn new(first: usize, last: usize) -> Segment {
        assert!(first <= last);
        Segment { first, last }
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    x: Segment,
    y: Segment,
    z: Segment,
}

impl Brick {
    fn parse(input: &str) -> Brick {
        let mut parts = input.split('~');
        let (lower_x, lower_y, lower_z) = parse_coord(parts.next().unwrap());
        let (upper_x, upper_y, upper_z) = parse_coord(parts.next().unwrap());
        Brick {
            x: Segment::new(lower_x, upper_x),
            y: Segment::new(lower_y, upper_y),
            z: Segment::new(lower_z, upper_z),
        }
    }
}

fn parse_coord(input: &str) -> (usize, usize, usize) {
    let mut parts = input.split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut bricks = INPUT.lines().map(Brick::parse).collect::<Vec<_>>();
    bricks.sort_by_key(|brick| brick.z.first);
    let max_x = bricks.iter().map(|b| b.x.last).max().unwrap();
    let max_y = bricks.iter().map(|b| b.y.last).max().unwrap();

    let mut support_graph = Dag::<(), ()>::new();
    let ground_node = support_graph.add_node(());
    let mut map = vec![vec![(0usize, ground_node); max_x + 1]; max_y + 1];

    for brick in bricks {
        // Find the support height and support nodes.
        let mut support_height = 0;
        let mut support_nodes = [ground_node].into_iter().collect::<HashSet<NodeIndex>>();
        for y in brick.y.first..=brick.y.last {
            for x in brick.x.first..=brick.x.last {
                let (test_height, test_node) = map[y][x];
                if test_height == support_height {
                    support_nodes.insert(test_node);
                } else if test_height > support_height {
                    support_height = test_height;
                    support_nodes = [test_node].into_iter().collect::<HashSet<NodeIndex>>();
                }
            }
        }

        // Calculate new height and node.
        let new_height = support_height + brick.z.last - brick.z.first + 1;
        let new_node = support_graph.add_node(());

        // Update map.
        for y in brick.y.first..=brick.y.last {
            for x in brick.x.first..=brick.x.last {
                map[y][x] = (new_height, new_node);
            }
        }

        // Update support graph.
        for support_node in support_nodes {
            support_graph.add_edge(support_node, new_node, ()).unwrap();
        }
    }

    let mut count = 0;
    for node in support_graph.node_identifiers() {
        if node == ground_node {
            continue;
        }

        if fallers(&support_graph, node).is_empty() {
            count += 1;
        }
    }
    println!("Part 1: {}", count);

    let num_bricks = support_graph.node_count() - 1;
    let mut result = 0;
    for node in support_graph.node_identifiers() {
        if node == ground_node {
            continue;
        }

        let mut disintegrated = support_graph.clone();
        disintegrated.remove_node(node);
        let num_bricks_left = Bfs::new(&disintegrated, ground_node).iter(&disintegrated).count() - 1;
        result += num_bricks - num_bricks_left - 1;
    }
    println!("Part 2: {}", result);
}

fn fallers(support_graph: &Dag<(), ()>, node: NodeIndex) -> Vec<NodeIndex> {
    let mut result = Vec::new();
    for edge in support_graph.edges_directed(node, Direction::Outgoing) {
        if support_graph
            .edges_directed(edge.target(), Direction::Incoming)
            .count()
            == 1
        {
            result.push(edge.target());
        }
    }
    result
}
