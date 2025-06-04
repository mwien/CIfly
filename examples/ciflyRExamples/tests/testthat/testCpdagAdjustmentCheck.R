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

test_that("Perkovic et al. (2015/2017a), Example 4.1/2", {
	cpdag <- list("-->" = rbind(c(1, 3), c(2, 3), c(3, 6), c(4, 3), c(4, 6), c(5, 6)), "---" = rbind(c(1, 2), c(2, 4), c(2, 5), c(4, 5)))
	x <- 3
	y <- 6

	expect_equal(isCpdagAdjustment(cpdag, x,y, c(2,4)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, c(4,5)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, c(4,2,1)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, c(4,5,1)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, c(4,2,5)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, c(4,2,5,1)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, 2), FALSE)
	expect_equal(isCpdagAdjustment(cpdag, x,y, NULL), FALSE)
})

test_that("Perkovic et al. (2015/2017a), Example 4.4/8", {
	cpdag <- list("-->" = rbind(c(2, 3), c(3, 5), c(4, 3)), "---" = rbind(c(1, 2)))
	x <- c(1,5)
	y <- 4
	expect_equal(isCpdagAdjustment(cpdag, x, y, c(2,3)), TRUE)
	expect_equal(isCpdagAdjustment(cpdag, x, y, 2), FALSE)
})
