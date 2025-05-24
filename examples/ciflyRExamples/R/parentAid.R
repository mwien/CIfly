library("ciflyR")
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
possDescendants <- parseRuletable(file.path(ruletables, "possible_descendants_cpdag.txt"))
notAmenable <- parseRuletable(file.path(ruletables, "not_amenable_cpdag.txt"))
forbiddenPath <- parseRuletable(file.path(ruletables, "forbidden_path_connected_cpdag.txt"))
nonCausal <- parseRuletable(file.path(ruletables, "non_causal_connected_cpdag.txt"))

vectorToIndicator <- function(p, l) {
	b <- rep(FALSE, p)
	b[l] <- TRUE
	return (b)
}

reachPossDescendants <- function(p, g, t) {
	return (vectorToIndicator(p, reach(g, list("X" = t, "W" = c()), possDescendants)))
}

reachNotAmenable <- function(p, g, t) {
	return (vectorToIndicator(p, reach(g, list("X" = t), notAmenable)))
}

reachForbiddenPath <- function(p, g, t, Z) {
	return (vectorToIndicator(p, reach(g, list("X" = t, "Z" = Z), forbiddenPath)))
}

reachNonCausal <- function(p, g, t, Z) {
	return (vectorToIndicator(p, reach(g, list("X" = t, "Z" = Z), nonCausal)))
}

notAmenablenotAdjustment <- function(p, g, t, Z) {
	nam <- reachNotAmenable(p, g, t)
	forb <- reachForbiddenPath(p, g, t, Z)
	ncau <- reachNonCausal(p, g, t, Z)

	nad <- rep(FALSE, p)
	nad[nam] <- TRUE
	nad[forb] <- TRUE
	nad[ncau] <- TRUE

	return (list("nam" = nam, "nad" = nad))
}

# This implementation is focused on performance and not clarity 
# It aims to achieve run-time comparable to parent_aid in Python and gadjid
parentAid <- function(p, gTrue, gGuess) {
	gTrueParsed <- parseGraph(gTrue, possDescendants)
	gGuessParsed <- parseGraph(gGuess, possDescendants)

	paList <- parentsCifly(p, gGuess)

	mistakes <- 0
	for (t in seq(p)) {
		pa <- vectorToIndicator(p, paList[[t]])
		namGuess <- reachNotAmenable(p, gGuessParsed, t)
		desTrue <- reachPossDescendants(p, gTrueParsed, t)
		namnad <- notAmenablenotAdjustment(p, gTrueParsed, t, paList[[t]])
		namTrue <- namnad$nam
		nadTrue <- namnad$nad

		for (y in seq(p)) {
			if (y == t) {
				next
			}
			if (pa[y]) {
				if (desTrue[y]) {
					mistakes <- mistakes + 1
				}
			} else if (namGuess[y]) {
				if (!namTrue[y]) {
					mistakes <- mistakes + 1
				}
			} else {
				if (nadTrue[y]) {
					mistakes <- mistakes + 1
				}
			}
		}
	}
	return (mistakes)
}
