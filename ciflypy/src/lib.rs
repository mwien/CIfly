use std::collections::HashMap;
use std::collections::HashSet;

use pyo3::exceptions::{PyOSError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

#[pymodule]
fn ciflypy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reach, m)?)?;
    m.add_class::<Ruletable>()?;
    m.add_class::<Graph>()?;
    m.add_class::<Sets>()?;
    Ok(())
}

#[derive(FromPyObject)]
enum Edges {
    #[pyo3(transparent, annotation = "list")]
    List(Vec<(usize, usize)>),
    #[pyo3(transparent, annotation = "set")]
    Set(HashSet<(usize, usize)>),
}

#[derive(FromPyObject)]
enum NodeSet {
    #[pyo3(transparent, annotation = "unsigned int")]
    Int(usize),
    #[pyo3(transparent, annotation = "list")]
    List(Vec<usize>),
    #[pyo3(transparent, annotation = "set")]
    Set(HashSet<usize>),
}

/// Perform the CIfly algorithm specified in the passed ruletable.
///
/// Parameters:
///     graph: A dictionary mapping edge types to edge lists.
///     sets: A dictionary mapping set names to a list of elements.
///     ruletable: Path to ruletable file.
///     table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.
///     verbose: Optional keyword argument to enable logging. Default value is False.
///
/// Returns:
///     A list of all reachable nodes.
#[pyfunction]
#[pyo3(signature = (graph, sets, ruletable, *,  table_as_string=false, verbose=false))]
fn reach(
    graph: Bound<'_, PyAny>,
    sets: Bound<'_, PyAny>,
    ruletable: Bound<'_, PyAny>,
    table_as_string: bool,
    verbose: bool,
) -> PyResult<Vec<usize>> {
    let settings = cifly::Settings::new(verbose, false);

    let borrow_ruletable;
    let parsed_ruletable;
    let ruletable_ref = if let Ok(rt) = ruletable.downcast::<Ruletable>() {
        borrow_ruletable = rt.borrow();
        &borrow_ruletable.0
    } else if let Ok(rt) = ruletable.str() {
        parsed_ruletable = to_ruletable(&rt, table_as_string)?;
        &parsed_ruletable
    } else {
        return Err(PyRuntimeError::new_err(
            "error reading ruletable: ruletable is neither a Ruletable object nor can be converted to a String"
                .to_owned(),
        ));
    };

    let borrow_graph;
    let parsed_graph;
    let graph_ref = if let Ok(g) = graph.downcast::<Graph>() {
        borrow_graph = g.borrow();
        &borrow_graph.0
    } else if let Ok(g) = graph.downcast::<PyDict>() {
        parsed_graph = to_graph(g, ruletable_ref)?;
        &parsed_graph
    } else {
        return Err(PyRuntimeError::new_err(
            "error reading graph: graph is neither a String nor a Graph object".to_owned(),
        ));
    };

    let borrow_sets;
    let parsed_sets;
    let sets_ref = if let Ok(s) = sets.downcast::<Sets>() {
        borrow_sets = s.borrow();
        &borrow_sets.0
    } else if let Ok(s) = sets.downcast::<PyDict>() {
        parsed_sets = to_sets(s, ruletable_ref)?;
        &parsed_sets
    } else {
        return Err(PyRuntimeError::new_err(
            "error reading sets: sets is neither a String nor a Sets object".to_owned(),
        ));
    };

    let reached = cifly::reach::reach(graph_ref, sets_ref, ruletable_ref, &settings);

    Ok(reached)
}

/// Constructs an internal CIfly ruletable representation. Mostly recommended for improving performance if the same ruletable is used multiple times.
///
/// Parameters:
///     ruletable: Path to ruletable file.
///     table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.
///
/// Returns:
///     Internal CIfly representation of a ruletable. This object can be passed instead of a file path to all methods with a ruletable argument.
#[pyclass]
struct Ruletable(cifly::Ruletable);

#[pymethods]
impl Ruletable {
    #[pyo3(signature = (ruletable, *, table_as_string=false))]
    #[new]
    fn new(ruletable: Bound<'_, PyAny>, table_as_string: bool) -> PyResult<Self> {
        if let Ok(ruletable) = ruletable.str() {
            let ruletable = to_ruletable(&ruletable, table_as_string)?;
            Ok(Ruletable(ruletable))
        } else {
            Err(PyRuntimeError::new_err(
                "error reading ruletable: ruletable cannot be converted to a String".to_owned(),
            ))
        }
    }
}

/// Constructs an internal CIfly graph representation. Mostly recommended for improving performance if the same graph is used multiple times.
///
/// Parameters:
///     graph: A dictionary mapping edge types to edge lists.
///     ruletable: Path to ruletable file.
///     table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.
///
/// Returns:
///     Internal CIfly representation of a graph. This object can be passed to all methods with a graph argument.
#[pyclass]
struct Graph(cifly::Graph);

#[pymethods]
impl Graph {
    #[pyo3(signature = (graph, ruletable, *, table_as_string=false))]
    #[new]
    fn new(
        graph: Bound<'_, PyDict>,
        ruletable: Bound<'_, PyAny>,
        table_as_string: bool,
    ) -> PyResult<Self> {
        let borrow_ruletable;
        let parsed_ruletable;
        let ruletable_ref = if let Ok(rt) = ruletable.downcast::<Ruletable>() {
            borrow_ruletable = rt.borrow();
            &borrow_ruletable.0
        } else if let Ok(rt) = ruletable.str() {
            parsed_ruletable = to_ruletable(&rt, table_as_string)?;
            &parsed_ruletable
        } else {
            return Err(PyRuntimeError::new_err(
                "error reading ruletable: ruletable is neither a Ruletable object nor can be converted to a String"
                    .to_owned(),
            ));
        };
        Ok(Graph(to_graph(&graph, ruletable_ref)?))
    }
}

/// Constructs an internal CIfly sets representation. Mostly recommended for improving performance if the same sets are used multiple times.
///
/// Parameters:
///     sets: A dictionary mapping set names to a list of elements.
///     ruletable: Path to ruletable file.
///     table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.
///
/// Returns:
///     Internal CIfly representation of sets. This object can be passed to all methods with a sets argument.
#[pyclass]
struct Sets(cifly::Sets);

#[pymethods]
impl Sets {
    #[pyo3(signature = (sets, ruletable, *, table_as_string=false))]
    #[new]
    fn new(
        sets: Bound<'_, PyDict>,
        ruletable: Bound<'_, PyAny>,
        table_as_string: bool,
    ) -> PyResult<Self> {
        let borrow_ruletable;
        let parsed_ruletable;
        let ruletable_ref = if let Ok(rt) = ruletable.downcast::<Ruletable>() {
            borrow_ruletable = rt.borrow();
            &borrow_ruletable.0
        } else if let Ok(rt) = ruletable.str() {
            parsed_ruletable = to_ruletable(&rt, table_as_string)?;
            &parsed_ruletable
        } else {
            return Err(PyRuntimeError::new_err(
                "error reading ruletable: ruletable is neither a Ruletable object nor can be converted to a String"
                    .to_owned(),
            ));
        };
        Ok(Sets(to_sets(&sets, ruletable_ref)?))
    }
}

fn to_ruletable(
    ruletable_str: &Bound<'_, PyString>,
    as_string: bool,
) -> PyResult<cifly::Ruletable> {
    let ruletable_str: String = ruletable_str.extract()?;
    let ruletable = if as_string {
        cifly::Ruletable::from_multiline_string(&ruletable_str)
    } else {
        cifly::Ruletable::from_file(&ruletable_str)
    };
    match ruletable {
        Err(cifly::ReadRuletableError::IoError(e)) => Err(PyOSError::new_err(format!(
            "IO error reading ruletable from file {}. \n{}",
            ruletable_str, e
        ))),
        Err(cifly::ReadRuletableError::ParseError(msg)) => Err(PyRuntimeError::new_err(format!(
            "parsing error reading ruletable {}. \n{}",
            ruletable_str, msg
        ))),
        Ok(rt) => Ok(rt),
    }
}

fn to_graph(graph: &Bound<'_, PyDict>, ruletable: &cifly::Ruletable) -> PyResult<cifly::Graph> {
    let mut edge_lists = HashMap::new();
    for (edge_string, edges) in graph.iter() {
        let edge_string: String = edge_string.extract()?;
        let edges = match edges.extract::<Edges>()? {
            Edges::List(l) => l,
            Edges::Set(s) => Vec::from_iter(s),
        };
        edge_lists.insert(edge_string, edges);
    }
    cifly::Graph::new(&edge_lists, ruletable)
        .map_err(|err| PyRuntimeError::new_err(format!("Error reading graph. \n{}", err)))
}

fn to_sets(sets: &Bound<'_, PyDict>, ruletable: &cifly::Ruletable) -> PyResult<cifly::Sets> {
    let mut set_lists = HashMap::new();
    for (set_string, set) in sets.iter() {
        let set_string: String = set_string.extract()?;
        let set = match set.extract::<NodeSet>().unwrap() {
            NodeSet::Int(u) => vec![u],
            NodeSet::List(l) => l,
            NodeSet::Set(s) => Vec::from_iter(s),
        };
        set_lists.insert(set_string, set);
    }
    cifly::Sets::new(&set_lists, ruletable)
        .map_err(|err| PyRuntimeError::new_err(format!("Error reading sets. \n{}", err)))
}
