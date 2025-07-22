import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
# re-use the ancestor table for ADMGs
ancestors_table = cf.Ruletable(ruletables / "ancestors_admg.txt")
backdoor_connected_table = cf.Ruletable(ruletables / "backdoor_connected_dag.txt")
frontdoor_forbidden_table = cf.Ruletable(ruletables / "frontdoor_forbidden_dag.txt")
intercepted_paths_table = cf.Ruletable(ruletables / "intercepted_paths_dag.txt")


def frontdoor(g, X, Y, I, R):
    Zi = set(R).difference(cf.reach(g, {"X": X}, backdoor_connected_table))
    A = cf.reach(g, {"X": Y}, ancestors_table)
    Zii = set(Zi).difference(
        cf.reach(g, {"Y": Y, "A": A, "Z": Zi, "X": X}, frontdoor_forbidden_table)
    )
    if set(I).issubset(Zii) and not set(Y).intersection(
        cf.reach(g, {"X": X, "Z": Zii}, intercepted_paths_table)
    ):
        return Zii
    else:
        return None
