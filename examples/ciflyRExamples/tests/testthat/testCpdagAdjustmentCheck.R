source("../../R/cpdagAdjustmentCheck.R")
source("../../R/utils.R")

test_that("Figure 4, Example 3", {
	cpdag <- matrix(0, nrow = 5, ncol = 5)
	cpdag[1,2] = 1
	cpdag[1,3] = 1
	cpdag[1,5] = 1
	cpdag[5,1] = 1
	cpdag[2,3] = 1
	cpdag[3,2] = 1
	cpdag[4,2] = 1
	cpdag[4,3] = 1

	cpdag <- pcalgCpdagToCifly(t(cpdag))

	x <- c(1)
	y <- c(3)
	z <- c(4)

	expect_equal(isCpdagAdjustment(cpdag, x, y, z), TRUE)
})

test_that("Perkovic et al. (2015/2017a), Example 4.1/2", {
	mFig1 <- matrix(c(0,1,1,0,0,0, 1,0,1,1,1,0, 0,0,0,0,0,1,
	0,1,1,0,1,1, 0,1,0,1,0,1, 0,0,0,0,0,0), 6,6)
	mFig1 <- pcalgCpdagToCifly(mFig1)
	x <- 3; y <- 6

	expect_equal(isCpdagAdjustment(mFig1, x,y, c(2,4)), TRUE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, c(4,5)), TRUE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, c(4,2,1)), TRUE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, c(4,5,1)), TRUE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, c(4,2,5)), TRUE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, c(4,2,5,1)), TRUE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, 2), FALSE)
	expect_equal(isCpdagAdjustment(mFig1, x,y, NULL), FALSE)
})

test_that("Perkovic et al. (2015/2017a), Example 4.4/8", {
	mFig5a <- matrix(c(0,1,0,0,0, 1,0,1,0,0, 0,0,0,0,1, 0,0,1,0,0, 0,0,0,0,0), 5,5)
	mFig5a <- pcalgCpdagToCifly(mFig5a)
	x <- c(1,5); y <- 4
	expect_equal(isCpdagAdjustment(mFig5a, x, y, c(2,3)), TRUE)
	expect_equal(isCpdagAdjustment(mFig5a, x, y, 2), FALSE)
})
