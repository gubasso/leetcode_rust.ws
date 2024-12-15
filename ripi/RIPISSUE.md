# RIPISSUE

I had the following code challenge at turing....
given a pile of rows... in a tree format... like this:

```js
[
    [1],
    [1,1],
    [1,2,1],
    [1,3,3,1],
    [1,4,6,4,1],
]
```

Where each element of the row `n` corresponds to the sum of the two elements right above him (at the `n-1` row).

Build a function that returns the entire row at the position `n`.

```js
const row = n => {
    let res = []
    // solution here
    return res
}
```

Where `0 <= n <= 30`

Examples:

```
n = 0
output = [1]

n = 1
output = [1,1]

n = 3
output = [1,3,3,1]
```
