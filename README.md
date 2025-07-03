# CIfly

A declarative framework for designing efficient causal inference algorithms based on reductions to graph reachability specified by rule tables. For all information on CIfly, see our [website](https://cifly.pages.dev/).

## Installation

CIfly can be installed via pip in Python 

```bash
# Python installation with pip
pip install ciflypy
```

and from R-universe in R (a CRAN submission is planned). 

```r
# R installation via R-universe
install.packages("ciflyr", repos = c("https://mwien.r-universe.dev"))
```

In Python, the installation should work out-of-the-box without relying on any further dependencies. In R, if the package is build on your system, as is the case for Linux distributions, the [Rust toolchain](https://rustup.rs/) needs to be installed. 

## Citing CIfly

If you use CIfly in your scientific work, please cite [this paper](https://arxiv.org/abs/2506.15758) introducing CIfly and its theoretical foundations:
```bibtex
@article{cifly2025,
  author  = {Marcel Wien{"{o}}bst and Sebastian Weichwald and Leonard Henckel},
  title   = {Linear-Time Primitives for Algorithm Development in Graphical Causal Inference},
  journal = {arXiv preprint arXiv:2506.15758},
  year    = {2025}
}
```

## Example

As an introductory example, we show how to test d-separation with CIfly. The CIfly algorithm specified by the following rule table (saved in the file ```dsep.txt```) returns all nodes d-connected to set X given set Z.

```
EDGES --> <--
SETS X, Z
START <-- AT X
OUTPUT ...

--> | <-- | current in Z
... | ... | current not in Z
```

The rule table can be embedded into the code as a multi-line string or loaded via file path such as in the implementation below that tests d-separation using Python

```py
import ciflypy as cf

dsep_table_path = "./dsep.txt"

def test_dsep(G, x, y, Z):
      R = cf.reach(G, {"X": x, "Z": Z}, dsep_table_path)
      return y not in R

# for graph 0 -> 1 -> 2, test whether 0 is d-separated from 2 by 1
print(test_dsep({"-->": [(0, 1), (1, 2)]}, 0, 2, [1]))
```

and R.

```r
library("ciflyr")

dsepTablePath <- "./dsep.txt"

test_dsep <- function(G, x, y, Z) {
      R <- reach(G, list("X" = x, "Z" = Z), dsepTablePath)
      return (!(y %in% R))
}
# for graph 1 -> 2 -> 3, test whether 1 is d-separated from 3 by 2
print(test_dsep(list("-->" = rbind(c(1, 2), c(2, 3))), 1, 3, c(2)))
```

Find more details on this example and how to use CIfly [in our documentation](https://cifly.pages.dev/docs/).

## Support for other languages

Currently, CIfly is available in Python and R. Both [ciflypy](./ciflypy/) and [ciflyr](./ciflyr/) are wrappers for the [cifly](./cifly/) Rust library that implements the rule-table parser and the reachability algorithm. Thus, packages for other languages can be based on the same Rust core. If you are interested in support for a particular language or want to contribute a wrapper package, please open an issue or contact us directly. 
