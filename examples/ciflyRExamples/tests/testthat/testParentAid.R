library("here")
source(here("R", "parentAid.R"))

test_that("Simple DAG test", {
	dagTrue <- list("-->" = rbind(c(1, 2), c(2, 3), c(1, 4), c(4, 3)))
	dagGuess <- list("-->" = rbind(c(1, 2), c(2, 3), c(4, 1), c(4, 3)))

	expect_equal(parentAid(4, dagTrue, dagGuess), 5)
})

test_that("Simple CPDAG test", {
	cpdagTrue <- list("-->" = rbind(c(1, 2), c(3, 2), c(4, 2)), "---" = rbind(c(3, 4)))
	cpdagGuess <- list("-->" = rbind(c(1, 2), c(3, 2), c(1, 4), c(3, 4)), "---" = rbind(c(2, 4)))

	expect_equal(parentAid(4, cpdagTrue, cpdagGuess), 4)
})
