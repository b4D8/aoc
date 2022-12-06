
input <- readLines("../input")

solve <- function(stream, size = 4) {
  i <- 1
  res <- NA
  unlist(lapply(stream, function(stream) {
    window <- strsplit(stream, "")[[1]]
    max <- length(window) - size + 1
    while (i < max && is.na(res)) {
      end <- i + size - 1
      if (length(unique(window[i:end])) == size) {
        res <- end
      }
      i <- i + 1
    }
    res
  }))
}

sample <- c(
  "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
  "bvwbjplbgvbhsrlpgdmjqwftvncz",
  "nppdvjthqldpwncqszvftbrmjlhg",
  "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
  "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
)
testthat::expect_equal(solve(sample), c(7, 5, 6, 10, 11))
testthat::expect_equal(solve(sample, 14), c(19, 23, 23, 29, 26))

part1 <- solve(input)
testthat::expect_equal(part1, 1876)

part2 <- solve(input, 14)
testthat::expect_equal(part2, 2202)
