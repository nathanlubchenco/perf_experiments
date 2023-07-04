# Objective
This is a small project to learn a bit about rust in a hands on way.

Choices made here are more likely to be informed by wanting to learn about particular language features than necessarily thinking this is the "best" way to do things.

For example, I was interested in learning about clis, io, enums and benchmarking.

# Usage
## Generate some data:

* cargo run -- generate --number 1000000 
  - defaults to vectors of length 100 and range between 0.0 and 1.0 and filename "vectors.json"
  - all of these are configurable
  - this is the search space data
* cargo run -- generate --number 2 -f "target_vec.json"
  - similar defaults, but we override the filename
  - these are the vectors we'll be looking for the nearest neighbors of

## Use the find command
* cargo run -- find -i heap -k 2

## Run benchmarks

* cargo bench
  - requires that vectors.json and target_vec.json have been generated
  - TODO: make these names paramaterizable 

# Capabilities

Currently only supports
* a naive heap based approach of finding the top k nearest neighbors.
* a naive parallel sort approach

TODO -- provide support for additional NN impls.

# Results

For these approaches, perf scales well with k, which is expected since these approaches do most of their work
by analyzing the distance for all data points in the search space.  Maintaining a slightly larger heap introduces some overhead but since that scales logarithmically, it does reasonably well.

All tests were done on a 1 million size search space, which is unrealistically low for many use cases, but made things easy to work with in memory.

heap 10                 time:   [38.602 ms 38.756 ms 38.941 ms]
                        change: [-0.4012% +0.1945% +0.8314%] (p = 0.53 > 0.05)
                        No change in performance detected.

heap 100                time:   [39.221 ms 39.315 ms 39.434 ms]
                        change: [-0.7303% -0.2003% +0.2695%] (p = 0.45 > 0.05)
                        No change in performance detected.

heap 1000               time:   [39.327 ms 39.396 ms 39.492 ms]
                        change: [-0.1495% +0.1482% +0.4472%] (p = 0.35 > 0.05)
                        No change in performance detected.

heap 10000              time:   [40.967 ms 41.018 ms 41.105 ms]
                        change: [-0.4109% +0.0896% +0.4659%] (p = 0.75 > 0.05)
                        No change in performance detected.

parallel sort 10        time:   [84.301 ms 84.334 ms 84.369 ms]
                        change: [-0.6117% +0.0366% +0.5737%] (p = 0.91 > 0.05)
                        No change in performance detected.

parallel sort 100       time:   [84.476 ms 84.536 ms 84.629 ms]
                        change: [+0.5517% +0.8552% +1.1188%] (p = 0.00 < 0.05)
                        Change within noise threshold.

parallel sort 1000      time:   [84.644 ms 84.673 ms 84.703 ms]
                        change: [+0.9079% +1.1531% +1.2953%] (p = 0.00 < 0.05)
                        Change within noise threshold.

parallel sort 10000     time:   [86.772 ms 86.841 ms 86.921 ms]
                        change: [-0.3027% +0.3544% +0.8019%] (p = 0.25 > 0.05)
                        No change in performance detected.

Doing all the distances in parallel is not able to overcome the cost of sorting everything, which is as expected, mostly wanted to experiment with parallel processing rust. 

Approximate nearest neighbor methods that pre-process the search space should perform considerably better. 
