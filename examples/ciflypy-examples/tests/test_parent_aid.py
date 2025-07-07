import ciflypy_examples.parent_aid as parent_aid


def test_dag_dist():
    g_true = {"-->": [(0, 1), (1, 2), (0, 3), (3, 2)]}
    g_guess = {"-->": [(0, 1), (1, 2), (3, 0), (3, 2)]}

    assert parent_aid.parent_aid(4, g_true, g_guess) == 5


def test_cpdag_dist():
    g_true = {"-->": [(0, 1), (2, 1), (3, 1)], "---": [(2, 3)]}
    g_guess = {"-->": [(0, 1), (2, 1), (0, 3), (2, 3)], "---": [(1, 3)]}

    assert parent_aid.parent_aid(4, g_true, g_guess) == 4
