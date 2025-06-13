library("ciflyr")
library("here")

source(here("R", "utils.R"))

ruletables <- getRuletablePath()
ancestorsTable <- parseRuletable(file.path(ruletables, "ancestors_admg.txt"))
closureTable = parseRuletable(file.path(ruletables, "closure_admg.txt"))

findNearestSeparator <- function(g, X, Y, I, R) {
    A <- reach(g, list("X" = c(X, Y, I)), ancestorsTable)
    Z0 <- intersect(R, setdiff(A, c(X, Y)))
    Xstar <- reach(g, list("X" = X, "Z" = Z0, "A" = A), closureTable)
    if (length(intersect(Xstar, Y)) > 0) {
        return (NULL)
    }
    return (union(intersect(Z0, Xstar), I))
}
