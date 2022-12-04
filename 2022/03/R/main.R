
input <- readLines("../input")

priority <- function(char) {
  which(c(letters, LETTERS) == char)
}

part1 <- unlist(lapply(input, function(line) {
  item <- unlist(strsplit(line, ""))
  len <- length(item)
  mid <- len / 2
  rucksack <- list(
    a = unique(item[1:mid]),
    b = unique(item[(mid + 1):len])
  )
  common <- rucksack$a[which(rucksack$a %in% rucksack$b)]
  priority(common)
})) |> sum()

testthat::expect_equal(part1, 7737)

part2 <- unlist(lapply(which(1:(length(input) - 2) %% 3 == 1), function(i) {
  group <- lapply(input[i:(i + 2)], function(x) {
    unique(unlist(strsplit(x, "")))
  })
  common <- group[[1]][which(group[[1]] %in% group[[2]])]
  common <- common[which(common %in% group[[3]])]
  priority(common)
})) |> sum()

testthat::expect_equal(part2, 2697)
