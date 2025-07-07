library("ciflyr") 
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
notAmenable <- parseRuletable(file.path(ruletables, "not_amenable_cpdag.txt"))
possibleAncestors <- parseRuletable(file.path(ruletables, "possible_ancestors_cpdag.txt"))
possibleDescendants <- parseRuletable(file.path(ruletables, "possible_descendants_cpdag.txt"))
backdoorConnected <- parseRuletable(file.path(ruletables, "backdoor_connected_cpdag.txt"))

isCpdagAdjustment <- function(cpdag, X, Y, W) {
	nam <- reach(cpdag, list("X" = X), notAmenable)

	anc <- reach(cpdag, list("X" = Y, "W" = X), possibleAncestors)
	des <- reach(cpdag, list("X" = X), possibleDescendants)
	cn <- intersect(anc, des)
	forb <- reach(cpdag, list("X" = cn), possibleDescendants)

	bconn <- reach(cpdag, list("X" = X, "C" = cn, "W" = W), backdoorConnected)

	return (length(intersect(nam, Y)) == 0 && length(intersect(forb, W)) == 0 && length(intersect(bconn, Y)) == 0)
}
