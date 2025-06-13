import ciflypy as cf
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()
not_amenable = cf.Ruletable(str(ruletables / "not_amenable_cpdag.txt"))
possible_anc = cf.Ruletable(str(ruletables / "possible_ancestors_cpdag.txt"))
possible_des = cf.Ruletable(str(ruletables / "possible_descendants_cpdag.txt"))
backdoor_conn = cf.Ruletable(str(ruletables / "backdoor_connected_cpdag.txt"))

def is_cpdag_adjustment(g, X, Y, W):
    nam = cf.reach(g, {"X": X}, not_amenable)

    anc = cf.reach(g, {"X": Y, "W": X}, possible_anc)
    des = cf.reach(g, {"X": X, "W": []}, possible_des)
    cn = set(anc).intersection(des)
    forb = cf.reach(g, {"X": cn}, possible_des)

    bconn = cf.reach(g, {"X": X, "C": cn, "W": W}, backdoor_conn)

    return (not set(nam).intersection(Y) 
            and not set(forb).intersection(W)
            and not set(bconn).intersection(Y))
