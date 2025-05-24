source("../../R/parentAid.R")

test_that("Simple DAG test", {
	dagTrue <- matrix(0, nrow = 4, ncol = 4)
	dagTrue[1,2] = 1
	dagTrue[2,3] = 1
	dagTrue[1,4] = 1
	dagTrue[4,3] = 1
	dagTrue <- pcalgCpdagToCifly(t(dagTrue))

	dagGuess <- matrix(0, nrow = 4, ncol = 4)
	dagGuess[1,2] = 1
	dagGuess[2,3] = 1
	dagGuess[4,1] = 1
	dagGuess[4,3] = 1

	dagGuess <- pcalgCpdagToCifly(t(dagGuess))

	expect_equal(parentAid(4, dagTrue, dagGuess), 5)
})

test_that("Simple CPDAG test", {
	cpdagTrue <- matrix(0, nrow = 4, ncol = 4)
	cpdagTrue[1,2] = 1
	cpdagTrue[3,2] = 1
	cpdagTrue[4,2] = 1
	cpdagTrue[3,4] = 1
	cpdagTrue[4,3] = 1
	cpdagTrue <- pcalgCpdagToCifly(t(cpdagTrue))

	cpdagGuess <- matrix(0, nrow = 4, ncol = 4)
	cpdagGuess[1,2] = 1
	cpdagGuess[3,2] = 1
	cpdagGuess[1,4] = 1
	cpdagGuess[3,4] = 1
	cpdagGuess[2,4] = 1
	cpdagGuess[4,2] = 1


	cpdagGuess <- pcalgCpdagToCifly(t(cpdagGuess))

	expect_equal(parentAid(4, cpdagTrue, cpdagGuess), 4)
})
