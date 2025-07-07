library("here")

source(here("R", "cpdagAdjustmentCheck.R"))
source(here("R", "utils.R"))

test_that("Figure 4, Example 3", {
	cpdag <- list("-->" = rbind(c(1, 2), c(1, 3), c(4, 2), c(4, 3)), "---" = rbind(c(1, 5), c(2, 3)))

	x <- c(1)
	y <- c(3)
	z <- c(4)

	expect_equal(isCpdagAdjustment(cpdag, x, y, z), TRUE)
})

