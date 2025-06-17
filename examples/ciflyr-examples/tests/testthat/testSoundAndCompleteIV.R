library("here")
source(here("R", "soundAndCompleteIV.R"))

# In many tests below there are other valid conditional instruments.
# The checked ones are which our algorithm should find in its current implementation.
# On unexpected test failure, please recheck manually or with an IV verifier.

test_that("Figure 5b", {
	p <- 5
	dirEdges <- rbind(c(1, 2), c(1, 4), c(2, 3), c(4, 5))
	bidirEdges <- rbind(c(3, 5), c(4, 5))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)
	x <- 4
	y <- 5

	iv <- soundAndCompleteInstrument(p, g, x, y)
	expect_equal(iv$Z, c(1))
	expect_equal(iv$W, numeric(0))
})

test_that("CIS Paper Appendix Figure 2", {
	p <- 5
	dirEdges <- rbind(c(1, 4), c(2, 1), c(2, 3), c(3, 1), c(4, 5))
	bidirEdges <- rbind(c(3, 5), c(4, 5))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)
	x <- 4
	y <- 5

	iv <- soundAndCompleteInstrument(p, g, x, y)
	expect_equal(iv$Z, c(1))
	expect_equal(sort(iv$W), c(2, 3))
})

test_that("CIS Paper Appendix Figure 2 With Mediator and Children", {
	p <- 9
	dirEdges <- rbind(c(1, 4), c(2, 1), c(2, 3), c(3, 1), c(4, 6), c(6, 5), c(4, 8), c(5, 9), c(6, 7))
	bidirEdges <- rbind(c(3, 5), c(4, 5))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)
	x <- 4
	y <- 5

	iv <- soundAndCompleteInstrument(p, g, x, y)
	expect_equal(iv$Z, c(1))
	expect_equal(sort(iv$W), c(2, 3))

})

test_that("CIS Paper Figure 3a", {
	p <- 4
	dirEdges <- rbind(c(1, 2), c(1, 3), c(3, 4))
	bidirEdges <- rbind(c(2, 4), c(3, 4))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)

	x <- 3
	y <- 4

	iv <- soundAndCompleteInstrument(p, g, x, y)
	expect_equal(iv$Z, c(1))
	expect_equal(iv$W, numeric(0))
})

test_that("CIS Paper Figure 3b", {
	p <- 4
	dirEdges <- rbind(c(1, 3), c(3, 2), c(4, 5))
	bidirEdges <- rbind(c(3, 4), c(4, 5))
	g <- list("-->" = dirEdges, "<->" = bidirEdges)

	x <- 4
	y <- 5

	iv <- soundAndCompleteInstrument(p, g, x, y)
	expect_equal(iv$Z, c(2))
	expect_equal(iv$W, numeric(0))
})
