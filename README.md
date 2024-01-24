# leetcode rust workspace

<!-- toc -->


## General

`icc_....`: interview crash course problems

issue tracker / todo list: https://github.com/driusan/bug

test command:
```
cargo nextest run -j 1 --no-fail-fast -p [package]
clrm; cargo test -p [package] -- --test-threads 1
clrm; cargo clippy --package [package]
```

## p2236_root_equals_sum_of_children

- implemented conversion of vector to treenode[^1]

## p28_find_the_index_of_the_first_occurrence_in_a_string
[Knuth-Morris-Pratt (KMP) algorithm | String Matching Algorithm | Substring Search](https://www.youtube.com/watch?v=4jY57Ehc14Y)
[Knuth–Morris–Pratt KMP - Find the Index of the First Occurrence in a String - Leetcode 28 - Python](https://www.youtube.com/watch?v=JoF0Z7nVSrA)

## p909_snakes_and_ladders
> optimization 
[Dijkstras Shortest Path Algorithm Explained | With Example | Graph Theory](https://www.youtube.com/watch?v=bZkzH5x0SKU)

## p997_find_the_town_judge
> graph

## References:

[^1]: [Construct a complete binary tree from given array in level order fashion](https://www.geeksforgeeks.org/construct-complete-binary-tree-given-array/)
