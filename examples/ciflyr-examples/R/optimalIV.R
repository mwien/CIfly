library("ciflyr")
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
descendantsTable <- parseRuletable(file.path(ruletables, "descendants_admg.txt"))
optimalIVTable <- parseRuletable(file.path(ruletables, "optimal_iv_admg.txt"))

# find Z, W according to the criterion given by Henckel et al. (2024)
# NOTE: the returned instrument is only optimal if there is no adjustment set between x and y
# we do NOT check this here and thus if this assumption is violated
# the function alone only ensures soundness, but not optimality
optimalInstrument <- function(g, x, y) {
	descendants <- reach(g, list("X" = x), descendantsTable)
	if (!(y %in% descendants)) {
		return (NULL)
	} 
	W <- reach(g, list("S" = y, "D" = descendants, "F" = x), optimalIVTable)
	Z <- setdiff(reach(g, list("S" = x, "D" = descendants, "F" = c()), optimalIVTable), W)

	if (length(Z) > 0 && (containsParent(g, x, Z) || containsSibling(g, x, Z))) {
		return (list("Z" = Z, "W" = W))
	} else {
		return (NULL)
	}
}
