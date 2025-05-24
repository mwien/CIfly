library("here")

# adapt this to the location of ruletables in your project
# note instead of using file paths, it is possible 
# to directly include the ruletable as a string in the source file
getRuletablePath <- function() {
	# we do NOT recommend having the ruletables outside the project directory
	# this is only done here to showcase the ruletables at the root of the repo
	return (here("..", "..", "ruletables"))
}

# vectorized function to compute the parents for each node in a CIfly graph
parentsCifly <- function(p, g) {
	dirEdges <- g[["-->"]]
	mat <- do.call(rbind, dirEdges)
	grouped <- split(mat[,1], mat[,2])
	result <- replicate(p, integer(0), simplify = FALSE)
	names(result) <- as.character(1:p)
	result[names(grouped)] <- grouped
	return (result)
}

# vectorized function to remove edges from a CIfly graph
removeEdgesCifly <- function(g, fromVars, toVars, edgeType) {
	p <- highestVertexCifly(g)
	fromVarsLogical <- rep(FALSE, p)
	fromVarsLogical[fromVars] <- TRUE
	toVarsLogical <- rep(FALSE, p)
	toVarsLogical[toVars] <- TRUE

	modEdges <- g[[edgeType]][sapply(g[[edgeType]], function (v) return (!fromVarsLogical[v[1]] || !toVarsLogical[v[2]]))]
	gNew <- g
	gNew[[edgeType]] <- modEdges
	return (gNew)
}
