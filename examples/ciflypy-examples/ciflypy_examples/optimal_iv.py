import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
descendants_table = cf.Ruletable(ruletables / "descendants_admg.txt")
optimal_iv_table = cf.Ruletable(ruletables / "optimal_iv_admg.txt")

# find Z, W according to the criterion given by Henckel et al. (2024)
# NOTE: the returned instrument is only optimal if there is no adjustment set between x and y
# we do NOT check this here and thus if this assumption is violated
# the function alone only ensures soundness, but not optimality
def optimal_instrument(g, x, y):
    descendants = cf.reach(g, {"X": x}, descendants_table)
    if y not in descendants:
        return None

    W = set(cf.reach(g, {"S": y, "D": descendants, "F": x}, optimal_iv_table))
    Z = set(cf.reach(g, {"S": x, "D": descendants, "F": []}, optimal_iv_table)) - W

    if Z and (utils.contains_parent(g, x, Z) or utils.contains_sibling(g, x, Z)):
        return (list(Z), list(W))
    else:
        return None
