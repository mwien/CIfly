library("ciflyr")
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

reachPossDescendants <- function(p, g, x) {
	return (vectorToIndicator(p, reach(g, list("X" = x, "W" = c()), possDescendants)))
}

reachNotAmenable <- function(p, g, x) {
	return (vectorToIndicator(p, reach(g, list("X" = x), notAmenable)))
}

reachForbiddenPath <- function(p, g, x, W) {
	return (vectorToIndicator(p, reach(g, list("X" = x, "W" = W), forbiddenPath)))
}

reachNonCausal <- function(p, g, x, W) {
	return (vectorToIndicator(p, reach(g, list("X" = x, "W" = W), nonCausal)))
}

notAmenablenotAdjustment <- function(p, g, x, W) {
	nam <- reachNotAmenable(p, g, x)
	forb <- reachForbiddenPath(p, g, x, W)
	ncau <- reachNonCausal(p, g, x, W)

	nad <- rep(FALSE, p)
	nad[nam] <- TRUE
	nad[forb] <- TRUE
	nad[ncau] <- TRUE

	return (list("nam" = nam, "nad" = nad))
}

parentAid <- function(p, gTrue, gGuess) {
	gTrueParsed <- parseGraph(gTrue, possDescendants)
	gGuessParsed <- parseGraph(gGuess, possDescendants)

	paList <- parents(p, gGuess)

	mistakes <- 0
	for (x in seq(p)) {
		pa <- vectorToIndicator(p, paList[[x]])
		namGuess <- reachNotAmenable(p, gGuessParsed, x)
		desTrue <- reachPossDescendants(p, gTrueParsed, x)
		namnadTrue <- notAmenablenotAdjustment(p, gTrueParsed, x, paList[[x]])

		for (y in seq(p)) {
			if (y == x) {
				next
			}
			if (pa[y]) {
				if (desTrue[y]) {
					mistakes <- mistakes + 1
				}
			} else if (namGuess[y]) {
				if (!namnadTrue$nam[y]) {
					mistakes <- mistakes + 1
				}
			} else {
				if (namnadTrue$nad[y]) {
					mistakes <- mistakes + 1
				}
			}
		}
	}
	return (mistakes)
}
