from ciflypy_examples.sound_and_complete_iv import sound_and_complete_instrument


def test_figure_5b():
    # labels = ["a", "b", "c", "x", "y"]
    p = 5
    admg = {"-->": [(0, 1), (0, 3), (1, 2), (3, 4)], "<->": [(2, 4), (3, 4)]}
    x = 3
    y = 4

    sac_iv = sound_and_complete_instrument(p, admg, x, y)
    assert sac_iv is not None
    assert set(sac_iv[0]) == {0}
    assert set(sac_iv[1]) == set()


def test_appendix_graphical_tools_for_cis_figure_2():
    # labels = ["a", "b", "c", "x", "y"]
    p = 5
    admg = {"-->": [(0, 3), (1, 0), (1, 2), (2, 0), (3, 4)], "<->": [(2, 4), (3, 4)]}
    x = 3
    y = 4

    sac_iv = sound_and_complete_instrument(p, admg, x, y)
    assert sac_iv is not None
    assert set(sac_iv[0]) == {0}
    assert set(sac_iv[1]) == {1, 2}


def test_appendix_graphical_tools_for_cis_figure_2_with_mediator_and_children():
    # labels = ["a", "b", "c", "x", "y", "m", "mc", "xc", "yc"]
    p = 9
    admg = {"-->": [(0, 3), (1, 0), (1, 2), (2, 0), (3, 5), (5, 4), (3, 7), (4, 8), (5, 6)], "<->": [(2, 4), (3, 4)]}
    x = 3
    y = 4

    sac_iv = sound_and_complete_instrument(p, admg, x, y)
    assert sac_iv is not None
    assert set(sac_iv[0]) == {0}
    assert set(sac_iv[1]) == {1, 2}


def test_appendix_graphical_tools_for_cis_figure_3a():
    # labels = ["a", "b", "x", "y"]
    p = 4
    admg = {"-->": [(0, 1), (0, 2), (2, 3)], "<->": [(1, 3), (2, 3)]}
    x = 2
    y = 3

    sac_iv = sound_and_complete_instrument(p, admg, x, y)
    assert sac_iv is not None
    assert set(sac_iv[0]) == {0}
    assert set(sac_iv[1]) == set()


def test_appendix_graphical_tools_for_cis_figure_3b():
    # labels = ["a", "b", "c" "x", "y"]
    p = 5
    admg = {"-->": [(0, 2), (2, 1), (3, 4)], "<->": [(1, 4), (2, 3), (3, 4)]}
    x = 3
    y = 4

    sac_iv = sound_and_complete_instrument(p, admg, x, y)
    assert sac_iv is not None
    assert set(sac_iv[0]) == {2}
    assert set(sac_iv[1]) == set()
