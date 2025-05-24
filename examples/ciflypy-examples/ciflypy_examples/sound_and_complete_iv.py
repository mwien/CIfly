import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
anc_table = cf.Ruletable(str(ruletables / "ancestors_admg.txt"))
desc_table = cf.Ruletable(str(ruletables / "descendants_admg.txt"))
closure_table = cf.Ruletable(str(ruletables / "closure_admg.txt"))
dsep_table = cf.Ruletable(str(ruletables / "dconnected_admg.txt"))


def causal_nodes(g, x, y):
    anc = cf.reach(g, {"X": y}, anc_table)
    des = cf.reach(g, {"X": x}, desc_table)
    return list(set(anc).intersection(des))


def forbidden(g, x, y):
    anc = cf.reach(g, {"X": y}, anc_table)
    des = cf.reach(g, {"X": x}, desc_table)
    cn = set(anc).intersection(des) - set([x])
    return cf.reach(g, {"X": cn}, desc_table) + [x]


def find_nearest_separator(g, x, y, r):
    # x and y are ints
    anc = cf.reach(g, {"X": [x, y]}, anc_table)
    z0 = set(r).intersection(set(anc) - set([x, y]))
    xstar = cf.reach(g, {"X": x, "Z": z0, "A": anc}, closure_table)
    if y in xstar:
        return None
    return list(z0.intersection(xstar))


def witness_ancestral_instrument(g, x, y, z, non_forb):
    w = find_nearest_separator(g, y, z, non_forb)
    if w is None or z not in cf.reach(g, {"X": x, "Z": w}, dsep_table):
        return None
    return w


def sound_and_complete_instrument(p, g, x, y):
    forb = forbidden(g, x, y)
    non_forb = set(range(p)) - set(forb)
    cn = set(causal_nodes(g, x, y))

    g_mod = utils.removed_edges(g, [x], cn, "-->")
    g_mod_parsed = cf.Graph(g_mod, anc_table)

    for z in non_forb:
        if z == y or z == x:
            continue
        w = witness_ancestral_instrument(g_mod_parsed, x, y, z, non_forb)
        if w is not None:
            return ([z], list(w))
    return None
