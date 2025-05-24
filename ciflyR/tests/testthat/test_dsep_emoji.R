dsepEmoji <- "
EDGES ➡️ ⬅️
SETS 🚦, 🧴
START ⬅️  AT 🚦
OUTPUT ...

➡️  | ⬅️  | current in 🧴
... | ... | current not in 🧴
"

test_that("dsep emoji pre-parsed: collider opened by child in Z", {
	edgelist <- list("➡️" = list(c(1, 2), c(3, 2), c(2, 4)))
	sets <- list("🚦" = c(1), "🧴" = c(4))
	
	ruletable <- parseRuletable(dsepEmoji, tableAsString=TRUE)
	graph <- parseGraph(edgelist, ruletable)
	sets <- parseSets(sets, ruletable)

	expect_equal(sort(reach(edgelist, sets, ruletable)), c(1, 2, 3, 4)) 
})

test_that("dsep emoji: collider not openend", {
	edgelist <- list("➡️" = list(c(1, 2), c(3, 2), c(2, 4)))
	sets <- list("🚦" = c(1), "🧴" = c())
	expect_equal(sort(reach(edgelist, sets, dsepEmoji, tableAsString=TRUE)), c(1, 2, 4)) 
})
