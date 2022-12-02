Day 1: Calorie Counting
================
2022-12-01

## Data

``` r
file <- "../input"
data <- readLines(file)
data <- as.integer(data)
head(data, 13)
```

    ##  [1] 3264 4043 2537 3319 2485 3218 5611 1753 7232 3265 1751 2233   NA

``` r
elf <- c(1, which(is.na(data)))
cal <- which(!is.na(data))
```

266 elves are carrying 2004 calories.

## Part One

We need to find the number of calories carried by the most loaded elf.

``` r
part_one <- 0
cal_per_elf <- sapply(seq_along(elf), function(i) {
  first <- elf[i] + 1
  last <- elf[i + 1]
  last <- ifelse(is.na(last), length(data), last - 1)
  cal_per_elf <- sum(data[first:last])
  if (cal_per_elf > part_one) {
    part_one <<- cal_per_elf
  }
  cal_per_elf
})
```

Answer is: 72070.

## Part Two

We need to find the number of calories carried by the 3 most loaded
elves.

``` r
n <- 3
cal_per_elf <- sort(cal_per_elf, decreasing = TRUE)
part_two <- sum(cal_per_elf[1:n])
```

Answer is: 211805.

## Refactor

``` r
desc_cum_sum <- function(file, n = 1) {
  cal <- scan(file, blank.lines.skip = FALSE, quiet = TRUE)
  elf <- cumsum(is.na(cal))
  cal_per_elf <- rowsum(cal, elf, na.rm = TRUE) |>
    sort(decreasing = TRUE)
  sum(cal_per_elf[1:n])
}

part_one <- desc_cum_sum(file)
part_two <- desc_cum_sum(file, 3)
```
