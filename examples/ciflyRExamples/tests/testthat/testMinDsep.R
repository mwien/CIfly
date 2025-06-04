library("here")
source(here("R", "minDsep.R"))

# figure from van der Zander (2020) paper
test_that("Figure 4 left", {
	dirEdges <- rbind(c(1, 3), c(2, 1), c(2, 3), c(3, 4), c(4, 5))
	g <- list("-->" = dirEdges)
	x <- 1
	y <- 5

	minDsep <- findMinSeparator(g, c(x), c(y), c(), seq(5))
	expect_equal(minDsep, c(3))
})

test_that("Figure 4 right", {
	dirEdges <- rbind(c(2, 1), c(2, 3), c(3, 5), c(4, 3), c(4, 5))
	bidirEdges <- rbind(c(1, 3))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)
	x <- 1
	y <- 5

	minDsep <- findMinSeparator(g, c(x), c(y), c(), seq(5))
	expect_equal(sort(minDsep), c(3, 4))
})
