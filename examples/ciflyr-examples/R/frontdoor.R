library("ciflyr")
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
# re-use the ancestor table for ADMGs
ancestorsTable = parseRuletable(file.path(ruletables, "ancestors_admg.txt"))
backdoorConnectedTable <- parseRuletable(file.path(ruletables, "backdoor_connected_dag.txt"))
frontdoorForbiddenTable <- parseRuletable(file.path(ruletables, "frontdoor_forbidden_dag.txt"))
interceptedPathsTable <- parseRuletable(file.path(ruletables, "intercepted_paths_dag.txt"))

frontdoor <- function(g, X, Y, I, R) {
	Zi <- setdiff(R, reach(g, list("X" = X), backdoorConnectedTable))
	A <- reach(g, list("X" = Y), ancestorsTable)
	Zii <- setdiff(Zi, reach(g, list("Y" = Y, "A" = A, "Z" = Zi, "X" = X), frontdoorForbiddenTable))
	if (setequal(I, intersect(I, Zii)) && length(intersect(Y, reach(g, list("X" = X, "Z" = Zii), interceptedPathsTable))) == 0) {
		return (Zii)
	} else {
		return (NULL)
	}
}
