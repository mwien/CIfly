from ciflypy_examples.min_dsep import find_min_separator


# figure from van der Zander (2020) paper
def test_figure_4_left():
    # labels = ["X", "V1", "V2", "V3", "Y"]
    admg = {"-->": [(0, 2), (1, 0), (1, 2), (2, 3), (3, 4)]}
    x = 0
    y = 4

    min_dsep = find_min_separator(admg, [x], [y], [], list(range(5)))
    assert min_dsep is not None
    assert set(min_dsep) == {2}


def test_figure_4_right():
    # labels = ["X", V1", "V2", "V3", "Y"]
    admg = {"-->": [(1, 0), (1, 2), (2, 4), (3, 2), (3, 4)], "<->": [(0, 2)]}
    x = 0
    y = 4

    min_dsep = find_min_separator(admg, [x], [y], [], list(range(5)))
    assert min_dsep is not None
    assert set(min_dsep) == {2, 3}
