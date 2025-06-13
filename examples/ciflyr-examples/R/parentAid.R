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

parentAid <- function(p, gTrue, gGuess) {
	gTrueParsed <- parseGraph(gTrue, possDescendants)
	gGuessParsed <- parseGraph(gGuess, possDescendants)

	paList <- parents(p, gGuess)

	mistakes <- 0
	for (t in seq(p)) {
		pa <- vectorToIndicator(p, paList[[t]])
		namGuess <- reachNotAmenable(p, gGuessParsed, t)
		desTrue <- reachPossDescendants(p, gTrueParsed, t)
		namnadTrue <- notAmenablenotAdjustment(p, gTrueParsed, t, paList[[t]])

		for (y in seq(p)) {
			if (y == t) {
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
