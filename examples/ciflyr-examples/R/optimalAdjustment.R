library("ciflyr") 
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
notAmenable <- parseRuletable(file.path(ruletables, "not_amenable_cpdag.txt"))
possibleAncestors <- parseRuletable(file.path(ruletables, "possible_ancestors_cpdag.txt"))
possibleDescendants <- parseRuletable(file.path(ruletables, "possible_descendants_cpdag.txt"))
parentsTable <- parseRuletable(file.path(ruletables, "parents_cpdag.txt"))

optimalAdjustment <- function(cpdag, X, Y){
	des <- reach(cpdag, list("X" = X), possibleDescendants)
	if (length(intersect(Y, des)) < length(Y)) {
		return (NULL)
	}
	
	nam <- reach(cpdag, list("X" = X), notAmenable)
	anc <- reach(cpdag, list("X" = Y, "W" = X), possibleAncestors)
	cn <- intersect(anc, des)
	forb <- reach(cpdag, list("X" = cn), possibleDescendants)
	if (length(intersect(forb, X)) != 0 | length(intersect(nam, Y)) != 0) {
		return (NULL)
	}
	
	pre_opt <- reach(cpdag, list("X" = cn), parentsTable)
	
	return (setdiff(pre_opt, union(forb, X)))
}
