from ciflypy_examples.nearest_dsep import find_nearest_separator

def find_min_separator(g, X, Y, I, R):
    Zx = find_nearest_separator(g, X, Y, I, R)
    if not Zx:
        return None
    Zy = find_nearest_separator(g, Y, X, I, Zx)
    if not Zy:
        return None
    return set(Zx).intersection(Zy).union(I)
