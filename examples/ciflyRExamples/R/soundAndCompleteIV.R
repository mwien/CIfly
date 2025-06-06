library("ciflyR")
library("here")

source(here("R", "utils.R"))
source(here("R", "nearestDsep.R"))

ruletables <- getRuletablePath()
ancTable <- parseRuletable(file.path(ruletables, "ancestors_admg.txt"))
desTable <- parseRuletable(file.path(ruletables, "descendants_admg.txt"))
dconTable <- parseRuletable(file.path(ruletables, "dconnected_admg.txt"))

soundAndCompleteInstrument <- function(p, g, x, y) {
	anc <- reach(g, list("X" = y), ancTable)
	des <- reach(g, list("X" = x), desTable)
	cn <- setdiff(intersect(anc, des), c(x))
	forb <- append(reach(g, list("X" = cn), desTable), x)
	notForb <- setdiff(1:p, forb)

	gParsed <- parseGraph(g, ancTable)
	gMod <- removeOrderedEdges(g, x, cn, "-->")
	gModParsed <- parseGraph(gMod, ancTable)
	
	for (z in setdiff(notForb, c(y))) {
		W = findNearestSeparator(gModParsed, c(y), c(z), c(), notForb)
		if (!is.null(W) && z %in% reach(gParsed, list("X" = x, "Z" = W), dconTable)) {
			return (list("Z" = z, "W" = W))
		}
	}
	return (NULL)
}
