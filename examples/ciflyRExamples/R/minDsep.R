library("here")

source(here("R", "nearestDsep.R"))

findMinSeparator <- function(g, X, Y, I, R) {
  Zx <- findNearestSeparator(g, X, Y, I, R)
  Zy <- findNearestSeparator(g, Y, X, I, Zx)
  
  if (is.null(Zx) || is.null(Zy)) {
    return(NULL)
  }
  
  return(union(intersect(Zx, Zy), I))
}
