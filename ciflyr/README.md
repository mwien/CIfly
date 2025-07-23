
<!-- README.md is generated from README.Rmd. Please edit that file -->

# ciflyr

<!-- badges: start -->

[![R-CMD-check](https://github.com/mwien/CIfly/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/mwien/CIfly/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

ciflyr is the R interface to the CIfly framework for designing efficient
causal inference algorithms based on reductions to graph reachability
specified by rule tables. For more information on CIfly, see our
[website](https://cifly.dev/) and our
[paper](https://arxiv.org/abs/2506.15758).

## Installation

``` r
install.packages("ciflyr")
```

Note that if the package is build from source on your system, as is the
case for Linux distributions, the [Rust toolchain](https://rustup.rs/)
needs to be installed.

## Usage

As a basic example, we show how to test d-separation with CIfly. The
CIfly algorithm specified by the rule table in the variable `dsepTable`
returns all nodes d-connected to the set of nodes `X` given set `Z`.
Based on this information, testing d-separation reduces to checking
membership in a vector.

``` r
library(ciflyr)

dsepTable <- "
EDGES --> <--
SETS X, Z
START <-- AT X
OUTPUT ...

--> | <-- | current in Z
... | ... | current not in Z
"

test_dsep <- function(G, x, y, Z) {
      R <- reach(G, list("X" = x, "Z" = Z), dsepTable, tableAsString=TRUE)
      return (!(y %in% R))
}
# for graph 1 -> 2 -> 3, test whether 1 is d-separated from 3 by 2
print(test_dsep(list("-->" = rbind(c(1, 2), c(2, 3))), 1, 3, c(2)))
#> [1] TRUE
```
