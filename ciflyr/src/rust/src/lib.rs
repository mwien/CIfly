#![allow(non_snake_case)]

use std::collections::HashMap;

use extendr_api::prelude::*;

extendr_module! {
    mod ciflyr;
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
/// @param graph A list mapping edge types to edge lists stored in matrix format.
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
/// edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
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
/// edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
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
/// edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
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
                    .expect("Error: expected a string as ruletable argument."),
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
/// edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
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
        if !v.is_null() && !v.is_matrix() {
            panic!("Error: each edge list should be given as x times 2 matrix of integers");
        }
        let edge_list = if !v.is_null() {
            if v.is_integer() {
                let edge_list_r: RMatrix<i32> = v
                    .as_matrix()
                    .expect("Error: edge list should be given as x times 2 matrix");
                let num_edges = edge_list_r.nrows();
                let edge_list_raw = edge_list_r.data();
                (0..num_edges)
                    .map(|i| {
                        (
                            i32_to_node_id(edge_list_raw[i]),
                            i32_to_node_id(edge_list_raw[i + num_edges]),
                        )
                    })
                    .collect()
            } else if v.is_real() {
                let edge_list_r: RMatrix<f64> = v
                    .as_matrix()
                    .expect("Error: edge list should be given as x times 2 matrix");
                let num_edges = edge_list_r.nrows();
                let edge_list_raw = edge_list_r.data();
                (0..num_edges)
                    .map(|i| {
                        (
                            f64_to_node_id(edge_list_raw[i]),
                            f64_to_node_id(edge_list_raw[i + num_edges]),
                        )
                    })
                    .collect()
            } else {
                panic!(
                    "Error: edge list matrix contains neither integers nor floating point numbers"
                )
            }
        } else {
            Vec::new()
        };
        edge_lists.insert(edge_string.to_string(), edge_list);
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
                .map(|&x| i32_to_node_id(x))
                .collect();
        } else if v.is_real() {
            s = v
                .as_real_slice()
                .unwrap()
                .iter()
                .map(|&x| f64_to_node_id(x))
                .collect();
        } else {
            panic!("Error: set vector contains neither integers nor floating point numbers");
        }
        set_lists.insert(set_string.to_string(), s);
    }
    cifly::Sets::new(&set_lists, ruletable).unwrap()
}

fn i32_to_node_id(id: i32) -> usize {
    if id >= 1 {
        (id as usize) - 1
    } else {
        panic!("Error: expected positive integer as node id")
    }
}

fn f64_to_node_id(id: f64) -> usize {
    let rounded_id = id.round();
    if rounded_id >= 1.0 {
        (rounded_id as usize) - 1
    } else {
        panic!("Error: expected positive integer as node id")
    }
}
