from ciflypy_examples.cpdag_adjustment_check import is_cpdag_adjustment


def test_figure4_example3():
    cpdag = {"-->": [(0, 1), (0, 2), (3, 1), (3, 2)], "---": [(0, 4), (1, 2)]}
    X = [0]
    Y = [2]
    Z = [3]

    assert is_cpdag_adjustment(cpdag, X, Y, Z)


def test_perkovic_example2():
    cpdag = {
        "-->": [(0, 2), (1, 2), (2, 5), (3, 2), (3, 5), (4, 5)],
        "---": [(0, 1), (1, 3), (1, 4), (3, 4)],
    }
    X = [2]
    Y = [5]

    assert is_cpdag_adjustment(cpdag, X, Y, [1, 3])
    assert is_cpdag_adjustment(cpdag, X, Y, [3, 4])
    assert is_cpdag_adjustment(cpdag, X, Y, [0, 1, 3])
    assert is_cpdag_adjustment(cpdag, X, Y, [0, 3, 4])
    assert is_cpdag_adjustment(cpdag, X, Y, [1, 3, 4])
    assert is_cpdag_adjustment(cpdag, X, Y, [0, 1, 3, 4])
    assert not is_cpdag_adjustment(cpdag, X, Y, [1])
    assert not is_cpdag_adjustment(cpdag, X, Y, [])


def test_perkovic_example8():
    cpdag = {"-->": [(1, 2), (2, 4), (3, 2)], "---": [(0, 1)]}
    X = [0, 4]
    Y = [3]

    assert is_cpdag_adjustment(cpdag, X, Y, [1, 2])
    assert not is_cpdag_adjustment(cpdag, X, Y, [2])
