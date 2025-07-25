use std::collections::HashMap;
use std::error::Error;
use std::{cmp, fmt};

use crate::ruletable::Ruletable;

/// Internal `Graph` representation.
///
/// # Usage
/// Can be constructed using [`Graph::new`] and then passed to `reach`.
pub struct Graph {
    n: usize,
    seps: Vec<usize>,
    vals: Vec<(usize, usize)>,
}

impl Graph {
    /// Constructs a new `Graph` from a map of edge types to edge lists.
    ///
    /// Each edge type (string) must be defined in the `Ruletable`.
    /// Each entry in `edge_lists` is a list of edges (u, v).
    ///
    /// Returns an error if any edge type is undefined in the rule table.
    pub fn new(
        edge_lists: &HashMap<String, Vec<(usize, usize)>>,
        ruletable: &Ruletable,
    ) -> Result<Graph, ParseGraphError> {
        let mut n = 0;
        for edges in edge_lists.values() {
            for &(u, v) in edges.iter() {
                n = cmp::max(n, cmp::max(u, v));
            }
        }
        n += 1;

        let mut degree = vec![0_usize; n];
        for edges in edge_lists.values() {
            for &(u, v) in edges.iter() {
                degree[u] += 1;
                degree[v] += 1;
            }
        }

        let mut seps = vec![0_usize; n + 1];
        for i in 0..n {
            seps[i + 1] = seps[i] + degree[i];
        }

        let mut vals = vec![(0_usize, 0_usize); seps[n]];
        let mut cursor = seps[..n].to_vec();

        for (edge_string, edges) in edge_lists.iter() {
            let (edge_num, edge_rev_num) = ruletable
                .get_edge_ids(edge_string)
                .ok_or(ParseGraphError(format!(
                    "edge {edge_string} was not specified in rule table"
                )))
                .unwrap();

            for &(u, v) in edges.iter() {
                let pos_u = cursor[u];
                vals[pos_u] = (v, edge_num);
                cursor[u] += 1;

                let pos_v = cursor[v];
                vals[pos_v] = (u, edge_rev_num);
                cursor[v] += 1;
            }
        }
        Ok(Graph { n, seps, vals })
    }

    pub(crate) fn num_vertices(&self) -> usize {
        self.n
    }

    pub(crate) fn neighbors(&self, u: usize) -> &[(usize, usize)] {
        let start = self.seps[u];
        let end = self.seps[u + 1];
        &self.vals[start..end]
    }
}

/// Error type for reporting invalid graph definitions.
#[derive(Debug)]
pub struct ParseGraphError(String);

impl fmt::Display for ParseGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error when parsing graph: {}", self.0)
    }
}

impl Error for ParseGraphError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Internal `Sets` representation.
///
/// # Usage
/// Can be constructed using [`Sets::new`] and then passed to `reach`.
pub struct Sets(Vec<Vec<bool>>);

impl Sets {
    /// Constructs a new `Sets` from a map of set labels to element lists.
    ///
    /// Each set label must be defined in the `Ruletable`.
    /// Duplicates within a set cause an error.
    pub fn new(
        sets: &HashMap<String, Vec<usize>>,
        ruletable: &Ruletable,
    ) -> Result<Sets, ParseSetsError> {
        let mut z = vec![Vec::new(); ruletable.num_sets()];
        for (set_string, set) in sets.iter() {
            let n = *set.iter().max().unwrap_or(&0) + 1;
            let set_num = ruletable
                .get_set_id(set_string)
                .ok_or(ParseSetsError(format!(
                    "set {set_string} was not specified in rule table"
                )))?;
            z[set_num] = vec![false; n];
            for &x in set.iter() {
                if z[set_num][x] {
                    return Err(ParseSetsError(format!(
                        "found duplicate entry {x} in set {set_string}"
                    )));
                }
                z[set_num][x] = true;
            }
        }
        Ok(Sets(z))
    }

    pub(crate) fn contains(&self, set_id: usize, element: usize) -> bool {
        if element >= self.0[set_id].len() {
            return false;
        }
        self.0[set_id][element]
    }

    pub(crate) fn elements(&self, set_id: usize) -> impl Iterator<Item = usize> + '_ {
        self.0[set_id]
            .iter()
            .enumerate()
            .filter(|(_, &val)| val)
            .map(|(i, _)| i)
    }

    pub(crate) fn max_size(&self) -> usize {
        self.0.iter().map(|s| s.len()).max().unwrap_or(0)
    }
}

/// Error type for reporting invalid set definitions.
#[derive(Debug)]
pub struct ParseSetsError(String);

impl fmt::Display for ParseSetsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error when parsing sets: {}", self.0)
    }
}

impl Error for ParseSetsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
