library("ciflyR")
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
ancTable <- parseRuletable(file.path(ruletables, "ancestors_admg.txt"))
desTable <- parseRuletable(file.path(ruletables, "descendants_admg.txt"))
closureTable <- parseRuletable(file.path(ruletables, "closure_admg.txt"))
dconTable <- parseRuletable(file.path(ruletables, "dconnected_admg.txt"))

causalNodes <- function(g, x, y) {
	anc <- reach(g, list("X" = y), ancTable)
	des <- reach(g, list("X" = x), desTable)
	return (intersect(anc, des))
}

forbidden <- function(g, x, y) {
	anc <- reach(g, list("X" = y), ancTable)
	des <- reach(g, list("X" = x), desTable)
	cn <- setdiff(intersect(anc, des), c(x))
	return (append(reach(g, list("X" = cn), desTable), x))
}

findNearestSeparator <- function(g, x, y, r) {
	anc <- reach(g, list("X" = c(x, y)), ancTable)
	z0 <- intersect(r, anc[!(anc %in% c(x, y))])
	xstar <- reach(g, list("X" = x, "Z" = z0, "A" = anc), closureTable)
	if (y %in% xstar) {
		return (NULL)
	}
	return (intersect(z0, xstar))
}

witnessAncestralInstrument <- function(g, x, y, z, notForb) {
	W <- findNearestSeparator(g, y, z, notForb)
	if (is.null(W) || !(z %in% reach(g, list("X" = x, "Z" = W), dconTable))) {
		return (NULL)
	}
	return (W)
}

soundAndCompleteInstrument <- function(p, g, x, y) {
	forb <- forbidden(g, x, y)
	notForb <- setdiff(1:p, forb)
	cn <- causalNodes(g, x, y)
	
	gMod <- removeEdges(g, x, cn, "-->")
	gModParsed <- parseGraph(gMod, ancTable)
	
	instruments <- list()
	numInstruments <- 1
	for (z in notForb) {
		# only relevant if y not a descendant of x
		if (z == y || z == x) {
			next
		}
		W = witnessAncestralInstrument(gModParsed, x, y, z, notForb)
		if (!is.null(W)) {
			instruments[[numInstruments]] <- list("Z" = z, "W" = W)
			numInstruments <- numInstruments + 1
		}
	}
	return (instruments)
}
