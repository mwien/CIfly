from pathlib import Path

# adapt this to the location of ruletables in your project
# note instead of using file paths, it is possible 
# to directly include the rule table as a string in the source file
def get_ruletable_path():
    # we do NOT recommend having the rule tables outside the project directory
    # this is only done here to showcase the ruletables at the root of the repo
    return Path(__file__).parent.parent.parent.parent.resolve() / "ruletables"
    
# an example function for removing certain edges from a CIfly graph
def removed_edges(g, from_vars, to_vars, edge_type):
    if edge_type not in g:
        return g

    from_set = set(from_vars)
    to_set = set(to_vars)
    gmod = dict(g)
    gmod[edge_type] = list(
        filter(
            lambda uv: uv[0] not in from_set or uv[1] not in to_set,
            g[edge_type],
        )
    )
    return gmod
