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
