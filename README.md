# rust algorithms and data structures

[![Coverage Status](https://coveralls.io/repos/github/xcaptain/rust-algorithms/badge.svg?branch=master)](https://coveralls.io/github/xcaptain/rust-algorithms?branch=master)
[![Build Status](https://travis-ci.com/xcaptain/rust-algorithms.svg?branch=master)](https://travis-ci.com/xcaptain/rust-algorithms)

referencing [https://github.com/trekhleb/javascript-algorithms](https://github.com/trekhleb/javascript-algorithms)

## algorithms

- [sort](algorithms/src/sort/mod.rs)

  - [bubble_sort](algorithms/src/sort/bubble_sort.rs)
  - [insertion_sort](algorithms/src/sort/insertion_sort.rs)
  - [selection_sort](algorithms/src/sort/selection_sort.rs)
  - [quick_sort](algorithms/src/sort/quick_sort.rs)
  - [merge_sort](algorithms/src/sort/merge_sort.rs)

- [search](algorithms/src/search/mod.rs)

  - [binary_search](algorithms/src/search/binary_search.rs)

- [math](algorithms/src/math/mod.rs)

  - [gcd](algorithms/src/math/gcd.rs)
  - [lcm](algorithms/src/math/lcm.rs)
  - [is_prime](algorithms/src/math/is_prime.rs)

- [misc](algorithms/src/misc/mod.rs)
  - [shortest_seq](algorithms/src/misc/shortest_seq.rs)
  - [two_sum](algorithms/src/misc/two_sum.rs)

## data structures

- [linkedlist](data-structures/src/linkedlist/mod.rs)

  - [simple_list](data-structures/src/linkedlist/SimpleList.rs)

- [stack](data-structures/src/stack/mod.rs)
  - [simple_stack](data-structures/src/stack/simple_stack.rs)

- [queue](data-structures/src/queue/mod.rs)
  - [mpsc_queue](data-structures/src/queue/mpsc_queue.rs)
  - [priority queue](data-structures/src/queue/priority_queue.rs)

- [tree](data-structures/src/tree/mod.rs)
  - [binary_tree](data-structures/src/tree/binary_tree.rs)

## project euler

This directory contains my solutions of the [https://projecteuler.net/](https://projecteuler.net/)

## how to build

### build the whole workspace

`cargo build`

### build per workspace

`cargo build -p algorithms`

`cargo build -p data-structures`

## how to test

### run all test cases

`cargo test`

### run specific test case

`cargo test two_sum_test`
