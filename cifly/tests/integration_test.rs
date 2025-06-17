use std::collections::HashMap;

#[test]
fn test_reach() {
    let ruletable_str = "
EDGES --> <--, ---
SETS X
COLORS init, yield
START ... [init] AT X
OUTPUT ... [yield]

... [init]  | ---      [yield] | next not in X
... [yield] | ---, --> [yield] | next not in X";
    let ruletable =
        cifly::Ruletable::from_multiline_string(ruletable_str).expect("should parse ruletable");

    let mut edge_lists = HashMap::new();
    edge_lists.insert("-->".to_owned(), vec![(2, 1), (2, 3), (3, 4), (5, 4)]);
    edge_lists.insert("---".to_owned(), vec![(0, 1), (0, 2)]);
    let graph = cifly::Graph::new(&edge_lists, &ruletable).expect("should parse graph");

    let mut sets = HashMap::new();
    sets.insert("X".to_owned(), vec![1]);
    let sets = cifly::Sets::new(&sets, &ruletable).expect("should parse sets");

    let settings = cifly::Settings::new(false, false);

    assert_eq!(
        cifly::reach::reach(&graph, &sets, &ruletable, &settings),
        vec![0, 2, 3, 4]
    );
}
