library("ciflyR")
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
desTable <- parseRuletable(file.path(ruletables, "descendants_admg.txt"))
optIVTable <- parseRuletable(file.path(ruletables, "optimal_iv_admg.txt"))

# find Z, W according to the criterion given by Henckel et al. (2024)
# NOTE: the returned instrument is only optimal if there is no adjustment set between x and y
# we do NOT check this here and thus if this assumption is violated
# the function alone only ensures soundness, but not optimality
optimalInstrument <- function(g, x, y) {
	des <- reach(g, list("X" = x), desTable)
	if (!(y %in% des)) {
		return (NULL)
	} 
	W <- reach(g, list("S" = y, "D" = des, "F" = x), optIVTable)
	Z <- setdiff(reach(g, list("S" = x, "D" = des, "F" = c()), optIVTable), W)

	if (length(Z) > 0) {
		return (list("Z" = Z, "W" = W))
	} else {
		return (NULL)
	}
}
