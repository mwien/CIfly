#![allow(non_snake_case)]

use std::collections::HashMap;

use extendr_api::prelude::*;

extendr_module! {
    mod ciflyR;
    fn reach;
    fn parseRuletable;
    fn parseGraph;
    fn parseSets;
    impl Ruletable;
    impl Graph;
    impl Sets;
}

/// Perform the CIfly algorithm specified in the passed ruletable.
///
/// For the given graph and sets, a CIfly reachability algorithm is run according to the ruletable specified in the ruletable argument. The algorithm returns all reachable nodes. It is guaranteed to run in linear-time.
///
/// @param graph A list mapping edge types to edge lists.
/// @param sets A list mapping set names to a list of elements.
/// @param ruletable Path to a ruletable file.
/// @param tableAsString Optional argument to enable passing the ruletable as multi-line string. Default value is FALSE.
/// @param verbose Optional argument to enable logging. Default value is FALSE.
/// @return A vector of all reachable nodes.
/// @examples
/// dsepTable <- "
///     EDGES --> <--
///     SETS X, Z
///     START <-- AT X
///     OUTPUT ...
///     --> | <-- | current in Z
///     ... | ... | current not in Z
/// "
///
/// edgelist <- list("-->" = list(c(1, 2), c(3, 2), c(2, 4)))
/// sets <- list("X" = c(1), "Z" = c(4))
/// reach(edgelist, sets, dsepTable, tableAsString=TRUE)
/// @export
#[extendr]
fn reach(
    graph: Robj,
    sets: Robj,
    ruletable: Robj,
    #[default = "FALSE"] tableAsString: bool,
    #[default = "FALSE"] verbose: bool,
) -> Vec<usize> {
    let settings = cifly::Settings::new(verbose, true);

    let parsed_ruletable;
    let ruletable_ref = match <&Ruletable>::try_from(&ruletable) {
        Ok(rt) => &rt.0,
        Err(_) => {
            parsed_ruletable = to_ruletable(
                ruletable
                    .as_str()
                    .expect("Error: expected a string as ruletable argument."),
                tableAsString,
            );
            &parsed_ruletable
        }
    };

    let parsed_graph;
    let graph_ref = match <&Graph>::try_from(&graph) {
        Ok(g) => &g.0,
        Err(_) => {
            parsed_graph = to_graph(&graph, ruletable_ref);
            &parsed_graph
        }
    };

    let parsed_sets;
    let sets_ref = match <&Sets>::try_from(&sets) {
        Ok(z) => &z.0,
        Err(_) => {
            parsed_sets = to_sets(&sets, ruletable_ref);
            &parsed_sets
        }
    };

    cifly::reach::reach(graph_ref, sets_ref, ruletable_ref, &settings)
        .iter()
        .map(|&x| x + 1)
        .collect()
}

#[extendr]
struct Ruletable(cifly::Ruletable);

#[extendr]
impl Ruletable {}

/// Obtain an internal representation of a CIfly ruletable.
///
/// Obtain an internal representation of a CIfly ruletable. Advanced usage only, mostly recommended for improving performance if the same ruletable is used multiple times. The parsed ruletable object can be passed to all methods with a ruletable argument.
///
/// @param ruletable Path to a ruletable file.
/// @param tableAsString Optional argument to enable passing the ruletable as multi-line string. Default value is FALSE.
/// @return Internal CIfly ruletable representation.
/// @examples
/// dsepTable <- "
///     EDGES --> <--
///     SETS X, Z
///     START <-- AT X
///     OUTPUT ...
///     --> | <-- | current in Z
///     ... | ... | current not in Z
/// "
///
/// rt <- parseRuletable(dsepTable, tableAsString=TRUE)
/// edgelist <- list("-->" = list(c(1, 2), c(3, 2), c(2, 4)))
/// sets <- list("X" = c(1), "Z" = c(4))
/// reach(edgelist, sets, rt)
/// @export
#[extendr]
fn parseRuletable(ruletable: Robj, #[default = "FALSE"] tableAsString: bool) -> Ruletable {
    Ruletable(to_ruletable(
        ruletable
            .as_str()
            .expect("Error: expected a string as ruletable argument."),
        tableAsString,
    ))
}

#[extendr]
struct Graph(cifly::Graph);

#[extendr]
impl Graph {}

/// Obtain an internal representation of a CIfly graph.
///
/// Obtain an internal representation of a CIfly graph. Advanced usage only, mostly recommended for improving performance if the same graph is used multiple times. The parsed graph object can be passed to all methods with a graph argument. It is compatible with all ruletables that have the same `EDGES ...` line as the ruletable passed as argument.
///
/// @param graph A list mapping edge types to edge lists.
/// @param ruletable Path to a ruletable file.
/// @param tableAsString Optional argument to enable passing the ruletable as multi-line string. Default value is FALSE.
/// @return Internal CIfly graph representation.
/// @examples
/// dsepTable <- "
///     EDGES --> <--
///     SETS X, Z
///     START <-- AT X
///     OUTPUT ...
///     --> | <-- | current in Z
///     ... | ... | current not in Z
/// "
/// edgelist <- list("-->" = list(c(1, 2), c(3, 2), c(2, 4)))
///
/// g <- parseGraph(edgelist, dsepTable, tableAsString=TRUE)
/// sets <- list("X" = c(1), "Z" = c(4))
/// reach(edgelist, sets, dsepTable, tableAsString=TRUE)
/// @export
#[extendr]
fn parseGraph(graph: Robj, ruletable: Robj, #[default = "FALSE"] tableAsString: bool) -> Graph {
    let parsed_ruletable;
    let ruletable_ref = match <&Ruletable>::try_from(&ruletable) {
        Ok(rt) => &rt.0,
        Err(_) => {
            parsed_ruletable = to_ruletable(
                ruletable
                    .as_str()
                    .expect("Error: xpected a string as ruletable argument."),
                tableAsString,
            );
            &parsed_ruletable
        }
    };
    Graph(to_graph(&graph, ruletable_ref))
}

#[extendr]
struct Sets(cifly::Sets);

#[extendr]
impl Sets {}

/// Obtain an internal representation of CIfly sets.
///
/// Obtain an internal representation of CIfly sets. Advanced usage only, mostly recommended for improving performance if the same sets are used multiple times. The parsed sets object can be passed to all methods with a sets argument. It is compatible with all ruletables that have the same `SETS ...` line as the ruletable passed as argument.
///
/// @param sets A list mapping set names to a list of elements.
/// @param ruletable Path to a ruletable file.
/// @param tableAsString Optional argument to enable passing the ruletable as multi-line string. Default value is FALSE.
/// @return Internal CIfly sets representation.
/// @examples
/// dsepTable <- "
///     EDGES --> <--
///     SETS X, Z
///     START <-- AT X
///     OUTPUT ...
///     --> | <-- | current in Z
///     ... | ... | current not in Z
/// "
/// sets <- list("X" = c(1), "Z" = c(4))
///
/// s <- parseSets(sets, dsepTable, tableAsString=TRUE)
/// edgelist <- list("-->" = list(c(1, 2), c(3, 2), c(2, 4)))
/// reach(edgelist, s, dsepTable, tableAsString=TRUE)
/// @export
#[extendr]
fn parseSets(sets: Robj, ruletable: Robj, #[default = "FALSE"] tableAsString: bool) -> Sets {
    let parsed_ruletable;
    let ruletable_ref = match <&Ruletable>::try_from(&ruletable) {
        Ok(rt) => &rt.0,
        Err(_) => {
            parsed_ruletable = to_ruletable(
                ruletable
                    .as_str()
                    .expect("Error: expected a string as ruletable argument."),
                tableAsString,
            );
            &parsed_ruletable
        }
    };
    Sets(to_sets(&sets, ruletable_ref))
}

fn to_ruletable(ruletable_str: &str, as_string: bool) -> cifly::Ruletable {
    let ruletable_res = if as_string {
        cifly::Ruletable::from_multiline_string(ruletable_str)
    } else {
        cifly::Ruletable::from_file(ruletable_str)
    };

    ruletable_res
        .map_err(|err| format!("Error: could not read ruletable {}: {}", ruletable_str, err))
        .unwrap()
}

fn to_graph(graph: &Robj, ruletable: &cifly::Ruletable) -> cifly::Graph {
    if !graph.is_list() {
        panic!("edges should be given as list");
    }
    let vecs = graph.as_list().unwrap().into_hashmap();
    let mut edge_lists = HashMap::new();
    for (edge_string, v) in vecs.iter() {
        if !v.is_list() && !v.is_null() {
            panic!("Error: each edge list should be as list");
        }
        let mut edges = Vec::new();
        if !v.is_null() {
            for tuple in v.as_list().unwrap().values() {
                if tuple.is_null() || tuple.is_empty() {
                    continue;
                }
                let e: Vec<usize>;
                if tuple.is_integer() {
                    e = tuple
                        .as_integer_slice()
                        .unwrap()
                        .iter()
                        .map(|&x| {
                            (x as usize)
                                .checked_sub(1)
                                .expect("Error: node ids have to be > 0")
                        })
                        .collect();
                } else if tuple.is_real() {
                    e = tuple
                        .as_real_slice()
                        .unwrap()
                        .iter()
                        .map(|&x| {
                            (x.round() as usize)
                                .checked_sub(1)
                                .expect("Error: node ids have to be > 0")
                        })
                        .collect();
                } else {
                    panic!("Error: vec is neither integer nor real");
                }
                if e.len() != 2 {
                    panic!("Error: each edge should consist of two vertices");
                }
                edges.push((e[0], e[1]));
            }
        }
        edge_lists.insert(edge_string.to_string(), edges);
    }
    cifly::Graph::new(&edge_lists, ruletable).unwrap()
}

fn to_sets(sets: &Robj, ruletable: &cifly::Ruletable) -> cifly::Sets {
    if !sets.is_list() {
        panic!("Error: sets should be given as list");
    }
    let vecs = sets.as_list().unwrap().into_hashmap();
    let mut set_lists = HashMap::new();
    for (set_string, v) in vecs.iter() {
        if !v.is_vector() && !v.is_number() && !v.is_null() {
            panic!("Error: each set should be a vector");
        }
        let s;
        if v.is_null() {
            s = Vec::new();
        } else if v.is_integer() {
            s = v
                .as_integer_slice()
                .unwrap()
                .iter()
                .map(|&x| {
                    (x as usize)
                        .checked_sub(1)
                        .expect("Error: node ids have to be > 0")
                })
                .collect();
        } else if v.is_real() {
            s = v
                .as_real_slice()
                .unwrap()
                .iter()
                .map(|&x| {
                    (x.round() as usize)
                        .checked_sub(1)
                        .expect("Error: node ids have to be > 0")
                })
                .collect();
        } else {
            panic!("vec is neither integer nor real");
        }
        set_lists.insert(set_string.to_string(), s);
    }
    cifly::Sets::new(&set_lists, ruletable).unwrap()
}
