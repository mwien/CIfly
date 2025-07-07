from ciflypy_examples.optimal_iv import optimal_instrument


def test_figure_5b():
    # labels = ["a", "b", "c", "x", "y"]
    admg = {"-->": [(0, 1), (0, 3), (1, 2), (3, 4)], "<->": [(2, 4), (3, 4)]}
    x = 3
    y = 4

    opt_iv = optimal_instrument(admg, x, y)
    assert opt_iv is not None
    assert set(opt_iv[0]) == {0}
    assert set(opt_iv[1]) == {1, 2}


def test_appendix_graphical_tools_for_cis_figure_2():
    # labels = ["a", "b", "c", "x", "y"]
    admg = {"-->": [(0, 3), (1, 0), (1, 2), (2, 0), (3, 4)], "<->": [(2, 4), (3, 4)]}
    x = 3
    y = 4

    opt_iv = optimal_instrument(admg, x, y)
    assert opt_iv is not None
    assert set(opt_iv[0]) == {0}
    assert set(opt_iv[1]) == {1, 2}


def test_appendix_graphical_tools_for_cis_figure_2_with_mediator_and_children():
    # labels = ["a", "b", "c", "x", "y", "m", "mc", "xc", "yc"]
    admg = {
        "-->": [(0, 3), (1, 0), (1, 2), (2, 0), (3, 5), (5, 4), (3, 7), (4, 8), (5, 6)],
        "<->": [(2, 4), (3, 4)],
    }
    x = 3
    y = 4

    opt_iv = optimal_instrument(admg, x, y)
    assert opt_iv is not None
    assert set(opt_iv[0]) == {0}
    assert set(opt_iv[1]) == {1, 2}


def test_appendix_graphical_tools_for_cis_figure_3a():
    # labels = ["a", "b", "x", "y"]
    admg = {"-->": [(0, 1), (0, 2), (2, 3)], "<->": [(1, 3), (2, 3)]}
    x = 2
    y = 3

    opt_iv = optimal_instrument(admg, x, y)
    assert opt_iv is None


def test_appendix_graphical_tools_for_cis_figure_3b():
    # labels = ["a", "b", "c" "x", "y"]
    admg = {"-->": [(0, 2), (2, 1), (3, 4)], "<->": [(1, 4), (2, 3), (3, 4)]}
    x = 3
    y = 4

    opt_iv = optimal_instrument(admg, x, y)
    assert opt_iv is None
