Day 1: Calorie Counting
================
2022-12-01

## Data

``` r
input <- readLines("../input")
input <- as.integer(input)
head(input, 13)
```

    ##  [1] 3264 4043 2537 3319 2485 3218 5611 1753 7232 3265 1751 2233   NA

``` r
elements <- length(input)
elf <- c(1, which(is.na(input)))
item <- which(!is.na(input))
```

We get this collection of 2269 elements:

- 2004 integers that represent calories contained in food items that are
  carried around by elves
- 265 missing values that delimit food distribution between 266 elves.

The goal of the puzzle is to find the elf carrying the most calories.

So we need to sum all calories contained in the food items carried by
each elf.

## Food distribution

``` r
shift <- c(elf[-1], elements)
dist <- abs(elf - shift)
summary(dist)
```

    ##    Min. 1st Qu.  Median    Mean 3rd Qu.    Max. 
    ##   2.000   5.000   8.000   8.526  12.750  16.000

## Calorie distribution

There is a total of 12 054 292 calories.

``` r
winner <- 0
payload <- sapply(seq_along(elf), function(i) {
  first <- elf[i] + 1
  last <- elf[i + 1]
  last <- ifelse(is.na(last), elements, last - 1)
  payload <- sum(input[first:last])

  if (payload > winner) {
    winner <<- payload
    names(winner) <<- i
  }

  payload
})

summary(payload)
```

    ##    Min. 1st Qu.  Median    Mean 3rd Qu.    Max. 
    ##    2366   39598   47235   45305   54171   72070

## Part One

The elf carrying the most calories is at index 40. He is carrying **72
070 calories**.

## Part Two

``` r
n <- 3
payload <- sort(payload, decreasing = TRUE)
answer <- sum(payload[1:n])
```

The 3 elves carrying the most calories are carrying **211 805
calories**.
