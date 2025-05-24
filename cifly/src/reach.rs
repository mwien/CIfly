use std::{cmp, collections::VecDeque};

use crate::{
    instance::{Graph, Sets},
    ruletable::Ruletable,
};

/// Configuration settings for running the reachability analysis.
#[derive(Debug, Default)]
pub struct Settings {
    /// Enables verbose output if true.
    pub verbose: bool,
    /// Formats output using one-indexed nodes if true.
    pub fmt_one_indexed: bool,
}

impl Settings {
    /// Creates a new `Settings` instance with the given verbosity and indexing format.
    pub fn new(verbose: bool, fmt_one_indexed: bool) -> Self {
        Self {
            verbose,
            fmt_one_indexed,
        }
    }
}

/// Computes the set of reachable nodes in a `graph` with respect to the provided `sets` and ruletable`.
///
/// # Arguments
/// - `graph`: The graph structure describing edges and neighbors.
/// - `sets`: The sets referred to in the ruletable.
/// - `ruletable`: Encodes rules for transitions, including start states and output states.
/// - `settings`: Controls verbosity and formatting.
///
/// # Returns
/// A `Vec<usize>` containing the node indices that are reachable and satisfy output constraints.
pub fn reach(graph: &Graph, sets: &Sets, ruletable: &Ruletable, settings: &Settings) -> Vec<usize> {
    let n = cmp::max(graph.num_vertices(), sets.max_size());
    let mut visited = Store::new(n, ruletable.num_edges(), ruletable.num_colors());
    let mut queue = VecDeque::new();

    if settings.verbose {
        let mut initial_states = Vec::new();
        for (set, e, c) in ruletable.starts().iter().copied() {
            for v in sets.elements(set) {
                let s = State {
                    node: v,
                    edge: e,
                    color: c,
                };
                initial_states.push(s.convert_to_string(ruletable, settings));
            }
        }
        println!("Initial States: {}", initial_states.join(", "));
    }

    let mut added = vec![false; n];

    let mut is_output = vec![vec![false; ruletable.num_colors()]; ruletable.num_edges()];
    for &(e, c) in ruletable.outputs() {
        is_output[e][c] = true;
    }
    let mut res = Vec::new();

    for (set, e, c) in ruletable.starts().iter().copied() {
        for v in sets.elements(set) {
            let s = State {
                node: v,
                edge: e,
                color: c,
            };
            if visited.contains(s) {
                continue;
            }
            visited.set(s);
            let pre_res = if s.node >= graph.num_vertices() {
                // initial sets may contain isolated vertices not present in the graph
                vec![s]
            } else {
                queue.push_back(s);
                bfs(graph, sets, ruletable, settings, &mut visited, &mut queue)
            };
            for s in pre_res {
                if added[s.node] || !is_output[s.edge][s.color] {
                    continue;
                }
                res.push(s.node);
                added[s.node] = true;
            }
        }
    }
    res
}

fn bfs(
    graph: &Graph,
    sets: &Sets,
    ruletable: &Ruletable,
    settings: &Settings,
    visited: &mut Store,
    queue: &mut VecDeque<State>,
) -> Vec<State> {
    let mut reached = Vec::new();
    while let Some(s1) = queue.pop_front() {
        reached.push(s1);
        if settings.verbose {
            println!(
                "Processing state {}",
                s1.convert_to_string(ruletable, settings)
            );
        }
        for (u2, t) in graph.neighbors(s1.node).iter().copied() {
            for &c2 in ruletable.possible_colors(s1.edge, s1.color, t).iter() {
                let s2 = State {
                    node: u2,
                    edge: t,
                    color: c2,
                };
                if visited.contains(s2) {
                    continue;
                }
                if ruletable.pass(sets, s1, s2) {
                    visited.set(s2);
                    queue.push_back(s2);
                    if settings.verbose {
                        println!(
                            "  Found transition '{}', add state '{}' to queue",
                            s1.convert_transition_to_string(&s2, ruletable, settings),
                            s2.convert_to_string(ruletable, settings),
                        );
                    }
                }
            }
        }
    }
    reached
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct State {
    pub node: usize,
    pub edge: usize,
    pub color: usize,
}

impl State {
    fn convert_to_string(&self, ruletable: &Ruletable, settings: &Settings) -> String {
        let edge_strings = ruletable.get_edge_strings();
        let color_strings = ruletable.get_color_strings();
        if color_strings.is_empty() {
            format!(
                "({}, {})",
                Self::convert_node_to_string(self.node, settings),
                edge_strings[self.edge]
            )
        } else {
            format!(
                "{}, {}, {}",
                Self::convert_node_to_string(self.node, settings),
                edge_strings[self.edge],
                color_strings[self.color]
            )
        }
    }
    fn convert_transition_to_string(
        &self,
        next: &State,
        ruletable: &Ruletable,
        settings: &Settings,
    ) -> String {
        let edge_strings = ruletable.get_edge_strings();
        let color_strings = ruletable.get_color_strings();
        if color_strings.is_empty() {
            format!(
                "{} {} {} {}",
                edge_strings[self.edge],
                Self::convert_node_to_string(self.node, settings),
                edge_strings[next.edge],
                Self::convert_node_to_string(next.node, settings)
            )
        } else {
            format!(
                "{} [{}] {} {} [{}] {}",
                edge_strings[self.edge],
                color_strings[self.color],
                Self::convert_node_to_string(self.node, settings),
                edge_strings[next.edge],
                color_strings[next.color],
                Self::convert_node_to_string(next.node, settings)
            )
        }
    }

    fn convert_node_to_string(u: usize, settings: &Settings) -> String {
        if settings.fmt_one_indexed {
            (u + 1).to_string()
        } else {
            u.to_string()
        }
    }
}

struct Store {
    v: Vec<bool>,
    num_edge_color: usize,
    num_colors: usize,
}

impl Store {
    fn new(num_vertices: usize, num_edges: usize, num_colors: usize) -> Store {
        Store {
            v: vec![false; num_vertices * num_colors * num_edges],
            num_edge_color: num_edges * num_colors,
            num_colors,
        }
    }

    fn contains(&self, s: State) -> bool {
        self.v[s.node * self.num_edge_color + s.edge * self.num_colors + s.color]
    }

    fn set(&mut self, s: State) {
        self.v[s.node * self.num_edge_color + s.edge * self.num_colors + s.color] = true;
    }
}
