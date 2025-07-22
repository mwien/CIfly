from ciflypy_examples.frontdoor import frontdoor


def test_figure_1i():
    # labels = ["X", "Z", "Y", "U"]
    g = {"-->": [(0, 1), (1, 2), (3, 0), (3, 2)]}
    x = [0]
    y = [2]
    fd = frontdoor(g, x, y, [], [1])
    assert fd is not None
    assert set(fd) == {1}


def test_figure_1ii():
    # labels = ["X", "A", "B", "C", "D", "Y", "U"]
    g = {"-->": [(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5), (6, 0), (6, 5)]}
    x = [0]
    y = [5]
    fd = frontdoor(g, x, y, [], [1, 2, 3, 4])
    assert fd is not None
    # not the only FD set, but algo returns maximum size one
    assert set(fd) == {1, 2, 3, 4}


def test_figure_1iii():
    # labels = ["X", "A", "B", "C", "D", "Y", "U"]
    g = {"-->": [(0, 1), (1, 5), (2, 5), (3, 5), (4, 2), (4, 3), (6, 0), (6, 5)]}
    x = [0]
    y = [5]
    fd = frontdoor(g, x, y, [], [1, 2, 3, 4])
    assert fd is not None
    assert set(fd) == {1, 2, 3, 4}


def test_no_fd():
    # labels = ["X", "Z", "Y", "U1", "U2"]
    g = {"-->": [(0, 1), (1, 2), (3, 0), (3, 2), (4, 1), (4, 2)]}
    x = [0]
    y = [2]
    fd = frontdoor(g, x, y, [], [1])
    assert fd is None


def test_figure_2():
    # labels = ["X", "A", "B", "C", "D", "Y", "U1", "U2", "U3", "U4"]
    g = {
        "-->": [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 5),
            (4, 1),
            (4, 9),
            (6, 0),
            (6, 5),
            (7, 0),
            (7, 3),
            (8, 2),
            (8, 5),
            (9, 5),
        ]
    }
    x = [0]
    y = [5]
    fd = frontdoor(g, x, y, [], [1, 2, 3, 4])
    assert fd is not None
    assert set(fd) == {1, 4}
