# in production code, prefer to specify file path with the "here" library
dsepTable <- "dsep.txt"

test_that("dsep: collider opened by child in Z", {
	edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
	sets <- list("X" = c(1), "Z" = c(4))
	expect_equal(sort(reach(edgelist, sets, dsepTable)), c(1, 2, 3, 4)) 
})

test_that("dsep: collider opened by child in Z with isolated nodes", {
	edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
	sets <- list("X" = c(1, 13), "Z" = c(4, 9))
	expect_equal(sort(reach(edgelist, sets, dsepTable)), c(1, 2, 3, 4, 13)) 
})

test_that("dsep: collider not opened", {
	edgelist <- list("-->" = rbind(c(1, 2), c(3, 2), c(2, 4)))
	sets <- list("X" = c(1), "Z" = c())
	expect_equal(sort(reach(edgelist, sets, dsepTable)), c(1, 2, 4)) 
})
