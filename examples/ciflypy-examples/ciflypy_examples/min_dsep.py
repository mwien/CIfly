from ciflypy_examples.nearest_dsep import find_nearest_separator

def find_min_separator(g, X, Y, I, R):
    Zx = find_nearest_separator(g, X, Y, I, R)
    Zy = find_nearest_separator(g, Y, X, I, Zx)
    if not Zx or not Zy:
        return None
    return set(Zx).intersection(Zy).union(I)
