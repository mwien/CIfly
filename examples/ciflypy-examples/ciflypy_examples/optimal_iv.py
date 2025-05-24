import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
desc_table = cf.Ruletable(str(ruletables / "descendants_admg.txt"))
opt_iv_table = cf.Ruletable(str(ruletables / "optimal_iv_admg.txt"))

# find Z, W according to the criterion given by Henckel et al. (2024)
# NOTE: the returned instrument is only optimal if there is no adjustment set between x and y
# we do NOT check this here and thus if this assumption is violated
# the function alone only ensures soundness, but not optimality
def optimal_instrument(g, x, y):
    de_x = cf.reach(g, {"X": x}, desc_table)
    if y not in de_x:
        return None

    w = set(cf.reach(g, {"S": y, "D": de_x, "F": x}, opt_iv_table))
    z = set(cf.reach(g, {"S": x, "D": de_x, "F": []}, opt_iv_table)) - w

    if z:
        return (list(z), list(w))
    else:
        return None
