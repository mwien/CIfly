import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
ancestors_table = cf.Ruletable(str(ruletables / "ancestors_admg.txt"))
closure_table = cf.Ruletable(str(ruletables / "closure_admg.txt"))

def find_nearest_separator(g, X, Y, I, R):
    A = cf.reach(g, {"X": X + Y + I}, ancestors_table)
    Z0 = set(R).intersection(set(A) - set(X + Y))
    Xstar = cf.reach(g, {"X": X, "Z": Z0, "A": A}, closure_table)
    if set(Xstar).intersection(Y):
        return None
    return list(Z0.intersection(Xstar).union(I))
