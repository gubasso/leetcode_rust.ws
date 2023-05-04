# leetcode rust workspace

<!-- toc GitLab -->

* [Todo](#todo)
* [General](#general)
* [p2236_root_equals_sum_of_children](#p2236_root_equals_sum_of_children)
* [p28_find_the_index_of_the_first_occurrence_in_a_string](#p28_find_the_index_of_the_first_occurrence_in_a_string)
* [p909_snakes_and_ladders](#p909_snakes_and_ladders)
* [p997_find_the_town_judge](#p997_find_the_town_judge)
* [References:](#references)

<!-- toc -->

## Todo

- [ ] [283. Move Zeroes](https://leetcode.com/problems/move-zeroes/): two-pointers
- [ ] [2000. Reverse Prefix of Word](https://leetcode.com/problems/reverse-prefix-of-word/): two-pointers
- [ ] [209. Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/): sliding window
- [ ] [1456. Maximum Number of Vowels in a Substring of Given Length](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/): sliding window
- [ ] [1208. Get Equal Substrings Within Budget](https://leetcode.com/problems/get-equal-substrings-within-budget/): sliding window
- [ ] [1732. Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/): Prefix sum
- [ ] [724. Find Pivot Index](https://leetcode.com/problems/find-pivot-index/): Prefix sum
- [ ] [303. Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/): Prefix sum

## General

`icc_....`: interview crash course problems

issue tracker / todo list: https://github.com/driusan/bug

test command:
```
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
