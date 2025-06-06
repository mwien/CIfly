import ciflypy as cifly
import ciflypy_examples.utils as utils

ruletables = utils.get_ruletable_path()

poss_desc_table = cifly.Ruletable(str(ruletables / "possible_descendants_cpdag.txt"))
not_amenable_table = cifly.Ruletable(str(ruletables / "not_amenable_cpdag.txt"))
forb_path_conn_table = cifly.Ruletable(str(ruletables / "forbidden_path_connected_cpdag.txt"))
non_causal_conn_table = cifly.Ruletable(str(ruletables / "non_causal_connected_cpdag.txt"))

def poss_desc(g, t):
    return set(cifly.reach(g, {"X": [t], "W": []}, poss_desc_table))

def not_amenable(g, t):
    return set(cifly.reach(g, {"X": t}, not_amenable_table))

def forbidden(g, t, Z):
    return set(cifly.reach(g, {"X": t, "Z": Z}, forb_path_conn_table))

def non_causal_connected(g, t, Z):
    return set(cifly.reach(g, {"X": t, "Z": Z}, non_causal_conn_table))

def not_amenable_not_adjustment(g, t, Z):
    nam = not_amenable(g, t)
    return nam, nam.union(forbidden(g, t, Z)).union(non_causal_connected(g, t, Z))

def parent_aid(p, g_true, g_guess):
    parents = [[] for _ in range(p)]
    for u, v in g_guess["-->"]:
        parents[v].append(u)
    g_guess_parsed = cifly.Graph(g_guess, poss_desc_table)
    g_true_parsed = cifly.Graph(g_true, poss_desc_table)
    mistakes = 0

    for t in range(p):
        pt = set(parents[t])
        not_amenable_guess = not_amenable(g_guess_parsed, t)
        desc_true = poss_desc(g_true_parsed, t)
        not_amenable_true, not_adjustment_true = not_amenable_not_adjustment(g_true_parsed, t, pt)

        for y in range(p):
            if y == t:
                continue
            if y in pt:
                if y in desc_true:
                    mistakes += 1
            elif y in not_amenable_guess:
                if y not in not_amenable_true:
                    mistakes += 1
            else:
                if y in not_adjustment_true:
                    mistakes += 1

    return mistakes
