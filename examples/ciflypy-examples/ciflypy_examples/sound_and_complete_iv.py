import ciflypy as cf
import ciflypy_examples.utils as utils
from ciflypy_examples.nearest_dsep import find_nearest_separator

ruletables = utils.get_ruletable_path()
anc_table = cf.Ruletable(ruletables / "ancestors_admg.txt")
desc_table = cf.Ruletable(ruletables / "descendants_admg.txt")
dsep_table = cf.Ruletable(ruletables / "dconnected_admg.txt")


def sound_and_complete_instrument(p, g, x, y):
    anc = cf.reach(g, {"X": y}, anc_table)
    des = cf.reach(g, {"X": x}, desc_table)
    cn = set(anc).intersection(des) - set([x])
    forb = cf.reach(g, {"X": cn}, desc_table) + [x]
    non_forb = set(range(p)) - set(forb)

    g_parsed = cf.Graph(g, anc_table)
    g_mod = utils.removed_ordered_edges(g, [x], cn, "-->")
    g_mod_parsed = cf.Graph(g_mod, anc_table)

    for z in non_forb - set([y]):
        W = find_nearest_separator(g_mod_parsed, [y], [z], [], non_forb)
        if W is not None and z in cf.reach(g_parsed, {"X": x, "Z": W}, dsep_table):
            return ([z], list(W))
    return None
