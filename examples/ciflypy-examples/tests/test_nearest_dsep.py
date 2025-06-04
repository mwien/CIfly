from ciflypy_examples.nearest_dsep import find_nearest_separator

# figure from van der Zander (2019) dissertation
def test_figure_6dot2_1():
    # labels = ["Y", "V1", "V2", "V3", "Z"]
    admg = {"-->": [(0, 2), (1, 0), (1, 2), (2, 3), (3, 4)]} 
    x = 0
    y = 4

    nearest_dsep = find_nearest_separator(admg, [x], [y], [], list(range(5)))
    assert nearest_dsep is not None
    assert set(nearest_dsep) == {1, 2} or set(nearest_dsep) == {2}

def test_figure_6dot2_2():
    # labels = ["Y", "V1", "V2", "Z"]
    admg = {"-->": [(1, 2), (1, 3), (2, 3)], "<->": [(0, 2)]} 
    x = 0
    y = 3

    nearest_dsep = find_nearest_separator(admg, [x], [y], [], list(range(4)))
    assert nearest_dsep is not None
    assert set(nearest_dsep) == {1, 2}

def test_figure_6dot2_3():
    # labels = ["Y", "V1", "V2", "V3", "V4", "V5", "Z"]
    admg = {"-->": [(1, 2), (2, 5), (2, 6), (3, 2), (4, 3), (4, 0)], "<->": [(0, 1), (0, 5)]} 
    x = 0
    y = 6

    nearest_dsep = find_nearest_separator(admg, [x], [y], [], list(range(6)))
    assert nearest_dsep is not None
    assert set(nearest_dsep) == {1, 4}
