# 876. Middle of the Linked List

<!-- toc GFM -->

+ [recursive node(n)](#recursive-noden)
+ [recursive node_until_n](#recursive-node_until_n)
    * [simple cases](#simple-cases)
+ [recursive structure](#recursive-structure)
+ [simple cases:](#simple-cases-1)
+ [transform: vec -> linked list](#transform-vec---linked-list)
    * [simple cases:](#simple-cases-2)

<!-- toc -->

# recursive node(n)

```

node(1) -> List { 
    val: 1,
    next: None,
}

node(2) -> List { 
    val: 1,
    next: List {
        val: 2,
        next: None,
    },
}

node(3) -> List { 
    val: 1,
    next: List {
        val: 2,
        next: List {
            val: 3,
            next: None,
        },
    },
}

node(n) -> List {
    val: 1,
    next: ... List {
        val: n,
        next: None,
    }
}


```



# recursive node_until_n

## simple cases

```
node_until_n(5, node_5()) -> List {
    val: 5,
    next: None,
}

node_until_n(4, node_5()) -> List {
    val: 5,
    next: List {
        val: 4,
        next: None,
    }
}

node_until_n(4, node_5()) -> List {
    val: 5,
    next: node_until_n(4, node_4())
}

node_until_n(3, node_5()) -> List {
    val: 5,
    next: List {
        val: 4,
        next: List {
            val: 3,
            next: None,
        }
    }
}

node_until_n(3, node_5()) -> List {
    val: 5,
    next: List {
        val: 4,
        next: node_until_n(3, node_3())
    }
}

node_until_n(3, node_5()) -> List {
    val: 5,
    next: node_until_n(3, node_4()),
}

---

node_until_n(n, node_m()) -> List {
    val: m,
    next: node_until_n(n, node_m-1())
}

```



# recursive structure

```
func(list_node) -> list_node {
    if (  ) {

    } else {

    }
}
```

# simple cases:

```
func( [1] ) -> [1]
func( [1,2] ) -> [1,2] -> [2].push( func( [1] ) )
func( [1,2,3] ) -> [1,2] -> func( [1,2] )
func( [1,2,3,4] ) -> [1,2,3] -> [3].push(func( [1,2] ))
func( [1,2,3,4,5] ) -> [1,2,3] -> func( [1,2,3,4] )
func( [1,2,3,4,5,6] ) -> [1,2,3,4] -> [4].push(func( [1,2,3,4,5] ))
func( [1,2,3,4,5,6,7] ) -> [1,2,3,4] -> func( [1,2,3,4,5,6] )
```

```
middle([1]) -> l {
    val: 1
    next: None
}

middle([1,2]) -> l {
    val: 2
    next: func([1])
}

middle([1,2,3]) -> l {
    val: 2
    next: func([1])
}

middle([1,2,3,4]) -> l {
    val: 3
    next: func([2])
}

middle([1,2,3,4,5]) -> l {
    val: 3
    next: func([2])
}

middle([1,2,3,4,5,6]) -> l {
    val: 4
    next: func([3])
}

middle([1,2,3,4,5,6,7]) -> l {
    val: 4
    next: func([3])
}

middle([1,2,3,4,5,6,7,8]) -> l {
    val: 5
    next: func([4])
}

middle([1,2,3,4,5,6,7,8,9]) -> l {
    val: 5
    next: func([4])
}

node_number = input_arr.len()/2 + 1

```

```
is_even = length % 2 == 0
if is_even {
    func(list_node) = func(list_node-1).push(node_last)
} else {
    func(list_node) = func(list_node-1)
}
```

```
func(list_node) -> list_node {
    if ( list_node == 1 ) {
        return [1]
    } else {
        is_even = length % 2 == 0
        if is_even {
            return func(list_node-1).push(node_last)
        } else {
            return func(list_node-1)
        }
    }
}
```

# transform: vec -> linked list

## simple cases:

```
transf( [1] ) -> ListNode { val: 1, next: None }
transf( [1,2] ) ->  ListNode { val: 2, next: transf([1]) }
transf( [1,2,3] ) ->  ListNode { val: 3, next: transf([1,2]) }
```

```
transf(vec) -> list_node {
    if ( vec.len() == 1 ) {
        return ListNode { val: vec[0], next: None }
    } else {
        return ListNode { val: vec.pop(), next: transf(vec) }
    }
}
```

