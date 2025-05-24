import ciflypy as cf
from pathlib import Path

dsep_table = str(Path(__file__).parent / "dsep.txt")


def test_dsep_collider_open():
    edgelist = {"-->": [(0, 1), (2, 1), (1, 3)]}
    sets = {"X": 0, "Z": [3]}

    reach_all = cf.reach(edgelist, sets, dsep_table)

    assert set(reach_all) == {0, 1, 2, 3}


def test_dsep_collider_open_with_isolated_nodes():
    edgelist = {"-->": [(0, 1), (2, 1), (1, 3)]}
    sets = {"X": [0, 12], "Z": [3, 8]}

    reach_all = cf.reach(edgelist, sets, dsep_table)

    assert set(reach_all) == {0, 1, 2, 3, 12}


def test_dsep_collider_blocked():
    edgeset = {"-->": {(0, 1), (2, 1), (1, 3)}}
    sets = {"X": set([0]), "Z": []}

    reach_three = cf.reach(edgeset, sets, dsep_table)

    assert set(reach_three) == {0, 1, 3}
