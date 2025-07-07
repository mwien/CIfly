from ciflypy_examples.optimal_adjustment import optimal_adjustment


def test_cpdag():
    g = {"-->": [(1, 4), (3, 4), (2, 4)], "---": [(0, 1), (0, 3), (1, 3)]}
    assert optimal_adjustment(g, [2], [4]) == {1, 3}
    assert optimal_adjustment(g, [3], [4]) is None
    assert optimal_adjustment(g, [4], [2]) is None


def test_dag():
    g = {"-->": [(0, 1), (1, 2), (2, 3), (2, 4)]}

    optadj = optimal_adjustment(g, [0], [3])
    assert optadj is not None
    assert len(optadj) == 0
    assert optimal_adjustment(g, [0, 4], [3]) is None
