library("here")

source(here("R", "optimalAdjustment.R"))
source(here("R", "utils.R"))

test_that("CPDAG", {
	cpdag <- list("-->" =rbind(c(2, 5), c(4, 5), c(3, 5)), "---" = rbind(c(1, 2), c(1,4), c(2,4)))

	expect_equal(sort(optimalAdjustment(cpdag, c(3), c(5))), c(2, 4))
	expect_equal(optimalAdjustment(cpdag, c(4), c(5)), NULL)
	expect_equal(optimalAdjustment(cpdag, c(5), c(3)), NULL)
})

test_that("DAG", {
	cpdag <- list("-->" =rbind(c(1, 2), c(2, 3), c(3, 4), c(3, 5)))

	expect_equal(length(optimalAdjustment(cpdag, c(1), c(4))), 0)
	expect_equal(optimalAdjustment(cpdag, c(1, 5), c(4)), NULL)
})
