library("here")
source(here("R", "frontdoor.R"))

test_that("Figure_1i", {
  # labels = c("X", "Z", "Y", "U") => now indexed 1 to 4
  g = list("-->" = rbind(c(1, 2), c(2, 3), c(4, 1), c(4, 3)))
  x <- c(1)
  y <- c(3)
  fd <- frontdoor(g, x, y, c(), c(2))
  expect_equal(sort(fd), c(2))
})

test_that("Figure_1ii", {
  # labels = c("X", "A", "B", "C", "D", "Y", "U") => 1 to 7
  g <- list("-->" = rbind(
    c(1, 2), c(2, 3), c(2, 4), c(3, 5), c(4, 5), c(5, 6), c(7, 1), c(7, 6)
  ))
  x <- c(1)
  y <- c(6)
  fd <- frontdoor(g, x, y, c(), c(2, 3, 4, 5))
  expect_equal(sort(fd), c(2, 3, 4, 5))
})

test_that("Figure_1iii", {
  # labels = c("X", "A", "B", "C", "D", "Y", "U") => 1 to 7
  g <- list("-->" = rbind(
    c(1, 2), c(2, 6), c(3, 6), c(4, 6), c(5, 3), c(5, 4), c(7, 1), c(7, 6)
  ))
  x <- c(1)
  y <- c(6)
  fd <- frontdoor(g, x, y, c(), c(2, 3, 4, 5))
  expect_equal(sort(fd), c(2, 3, 4, 5))
})

test_that("No Frontdoor Set", {
  # labels = c("X", "Z", "Y", "U1", "U2") => 1 to 5
  g <- list("-->" = rbind(
    c(1, 2), c(2, 3), c(4, 1), c(4, 3), c(5, 2), c(5, 3)
  ))
  x <- c(1)
  y <- c(3)
  fd <- frontdoor(g, x, y, c(), c(2))
  expect_null(fd)
})

test_that("Figure_2", {
  # labels = c("X", "A", "B", "C", "D", "Y", "U1", "U2", "U3", "U4") => 1 to 10
  g <- list("-->" = rbind(
    c(1, 2), c(2, 3), c(3, 4), c(4, 6), c(5, 2), c(5, 10), c(7, 1), c(7, 6),
    c(8, 1), c(8, 4), c(9, 3), c(9, 6), c(10, 6)
  ))
  x <- c(1)
  y <- c(6)
  fd <- frontdoor(g, x, y, c(), c(2, 3, 4, 5))
  expect_equal(sort(fd), c(2, 5))
})
