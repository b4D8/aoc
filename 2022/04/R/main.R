
input <- readLines("../input")

section <- lapply(input, function(line) {
  pair <- unlist(strsplit(line, ","))
  lapply(strsplit(pair, "-"), as.integer)
})

part1 <- unlist(lapply(section, function(x) {
  min(x[[1]]) <= min(x[[2]]) && max(x[[1]]) >= max(x[[2]]) ||
    min(x[[2]]) <= min(x[[1]]) && max(x[[2]]) >= max(x[[1]])
})) |> sum()

testthat::expect_equal(part1, 518)

part2 <- unlist(lapply(section, function(x) {
  min(x[[1]]) <= max(x[[2]]) && max(x[[1]]) >= min(x[[2]])
})) |> sum()

testthat::expect_equal(part2, 909)
