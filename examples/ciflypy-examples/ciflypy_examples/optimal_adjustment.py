import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
not_amenable_table = cf.Ruletable(ruletables / "not_amenable_cpdag.txt")
possible_ancestors_table = cf.Ruletable(ruletables / "possible_ancestors_cpdag.txt")
possible_descendants_table = cf.Ruletable(ruletables / "possible_descendants_cpdag.txt")
parents_table = cf.Ruletable(ruletables / "parents_cpdag.txt")


def optimal_adjustment(cpdag, X, Y):
    descendants = cf.reach(cpdag, {"X": X}, possible_descendants_table)
    if set(Y).difference(descendants):
        return None

    not_amenable = cf.reach(cpdag, {"X": X}, not_amenable_table)
    ancestors = cf.reach(cpdag, {"X": Y, "W": X}, possible_ancestors_table)
    cn = set(ancestors).intersection(descendants)
    forbidden = cf.reach(cpdag, {"X": cn}, possible_descendants_table)
    if set(forbidden).intersection(X) or set(Y).intersection(not_amenable):
        return None

    pre_opt = cf.reach(cpdag, {"X": cn}, parents_table)
    return set(pre_opt).difference(set(forbidden).union(X))
