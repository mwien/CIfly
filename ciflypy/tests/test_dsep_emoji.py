import ciflypy as cf

dsep_table = """
EDGES ➡️ ⬅️
SETS 🚦, 🧴
START ⬅️  AT 🚦
OUTPUT ...

➡️  | ⬅️  | current in 🧴
... | ... | current not in 🧴
"""

def test_dsep_emoji_collider_opened_by_child():
    edgelist = {"➡️": [(0, 1), (2, 1), (1, 3)]}
    sets = {"🚦": [0], "🧴": [3]}

    reach_all = cf.reach(edgelist, sets, dsep_table, table_as_string=True)

    assert set(reach_all) == {0, 1, 2, 3}


def test_dsep_emoji_collider_not_opened():
    edgeset = {"➡️": {(0, 1), (2, 1), (1, 3)}}
    sets = {"🚦": set([0]), "🧴": []}

    reach_three = cf.reach(edgeset, sets, dsep_table, table_as_string=True)

    assert set(reach_three) == {0, 1, 3}
