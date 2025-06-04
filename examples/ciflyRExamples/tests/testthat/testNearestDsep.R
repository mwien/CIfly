library("here")
source(here("R", "nearestDsep.R"))

# figure from van der Zander (2019) dissertation
test_that("Figure 6.2 G_1", {
	dirEdges <- rbind(c(1, 3), c(2, 1), c(2, 3), c(3, 4), c(4, 5))
	g <- list("-->" = dirEdges)
	x <- 1
	y <- 5

	nearestDsep <- findNearestSeparator(g, c(x), c(y), c(), seq(5))
	expect_equal(sort(nearestDsep), c(2, 3))
})

test_that("Figure 6.2 G_2", {
	dirEdges <- rbind(c(2, 3), c(2, 4), c(3, 4))
	bidirEdges <- rbind(c(1, 3))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)
	x <- 1
	y <- 4

	nearestDsep <- findNearestSeparator(g, c(x), c(y), c(), seq(4))
	expect_equal(sort(nearestDsep), c(2, 3))
})

test_that("Figure 6.2 G_3", {
	dirEdges <- rbind(c(2, 3), c(3, 5), c(3, 7), c(4, 3), c(5, 4), c(5, 1))
	bidirEdges <- rbind(c(1, 2), c(1, 6))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)
	x <- 1
	y <- 7

	nearestDsep <- findNearestSeparator(g, c(x), c(y), c(), seq(6))
	expect_equal(sort(nearestDsep), c(2, 5))
})
