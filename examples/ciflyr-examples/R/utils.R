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
# can be adapted to solve similar tasks
parents <- function(p, g) {
	dirEdges <- g[["-->"]]
	grouped <- split(dirEdges[, 1], dirEdges[, 2])
	result <- replicate(p, integer(0), simplify = FALSE)
	names(result) <- as.character(1:p)
	result[names(grouped)] <- grouped
	return (result)
}

# returns highest vertex which takes part in an edge
# if you need to access the number of vertices of the graph, 
# it is recommended to store that directly instead of using this function
# to ensure consistent handling of isolated vertices
highestNodeId <- function(g) {
	nodeIds <- unlist(g)
	if (length(nodeIds) == 0) {
		return (0)
	} else {
		return (max(nodeIds))
	}
}

# vectorized function to remove certain ordered edges from a CIfly graph
removeOrderedEdges <- function(g, fromVars, toVars, edgeType) {
	p <- highestNodeId(g)
	if (p == 0 || !(edgeType %in% names(g))) {
		return (g)
	}

	# enable efficient lookup of membership in fromVars and toVars
	fromVarsLogical <- rep(FALSE, p)
	fromVarsLogical[fromVars] <- TRUE
	toVarsLogical <- rep(FALSE, p)
	toVarsLogical[toVars] <- TRUE

	edgeList <- g[[edgeType]]
	edgeListNew <- edgeList[!fromVarsLogical[edgeList[,1]] | !toVarsLogical[edgeList[,2]], , drop=FALSE]
	gNew <- g
	gNew[[edgeType]] <- edgeListNew
	return (gNew)
}

containsParent <- function(g, x, Z) {
	p <- highestNodeId(g)

	# enable efficient lookup of membership in Z
	zLogical <- rep(FALSE, p)
	zLogical[Z] <- TRUE

	edgeList <- g[["-->"]]
	return (length(edgeList[zLogical[edgeList[,1]] & edgeList[,2] == x]) > 0)
}

containsSibling <- function(g, x, Z) {
	p <- highestNodeId(g)

	# enable efficient lookup of membership in Z
	zLogical <- rep(FALSE, p)
	zLogical[Z] <- TRUE

	edgeList <- g[["<->"]]
	return (length(edgeList[(zLogical[edgeList[,1]] & edgeList[,2] == x) | (zLogical[edgeList[,2]] & edgeList[,1] == x)]) > 0)
}
