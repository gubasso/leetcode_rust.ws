
```
"III"
```

- sum = 0

"I" (1º)
- num = 1
- counter_rep = 1
- condition:
    - index == 0
- exec:
    - bucket = 0 + 1 = 1

"I" (2º)
- num = 1
- counter_rep = 2
- condition:
    - 1 (last) == 1 (actual)
- exec:
    - bucket = 1 + 1 (bucket) = 2

"I" (3º)
- num = 1
- counter_rep = 3
- condition:
    - 1 (last) == 1 (actual)
- exec:
    - bucket = 1 + 2 (bucket) = 3
- condition:
    - index == c.len()
- exec:
    - sum = 0 (sum) + 3 (bucket) = 3

```
"XVI"
```

"X" (1º)
- num = 10
- counter_rep = 1
- condition:
    - index == 0
- exec:
    - bucket = 0 (bucket) + 10 (actual) = 10

"V" (2º)
- num = 5
- counter_rep = 1
- sum = 10 + 5 = 15
- condition:
    - 10 (last) > 5 (actual)
- exec:
    - sum = 0 (sum) + 10 (bucket) = 10
    - bucket = 0
    - bucket = 0 + 5 = 5

"I" (3º)
- num = 1
- counter_rep = 1
- sum = 15 + 1 = 16
- condition:
    - 5 (last) > 1 (actual)
- exec:
    - sum = 10 (sum) + 5 (bucket) = 15
    - bucket = 0
    - bucket = 0 (bucket) + 1 (actual) = 1
- condition:
    - index == c.len()
- exec:
    - sum = 15 (sum) + 1 (bucket) = 16

```
"IIIV"
```

- sum = 0

"I" (1º)
- num = 1
- counter_rep = 1
- sum = 0 + 1 = 1

"I" (2º)
- num = 1
- counter_rep = 2
- sum = 1 + 1 = 2

"I" (3º)
- num = 1
- counter_rep = 3
- sum = 2 + 1 = 3

"V" (4º)
- num = 5
- counter_rep = 1
- sum = 5 - 3 = 2

```
"MCMXCIV" (1994)
```

- total = MCMXCIV.len()
- sum = 0
- bucket = 0

"M" (1º)
- num = 1000
- counter_rep = 1
- condition:
    - index == 0
- exec:
    - bucket = 0 + 1000 = 1000

"C" (2º)
- num = 100
- counter_rep = 1
- condition:
    - 1000 (last) > 100 (actual)
- exec:
    - sum = 0 + 1000 (bucket) = 1000
    - bucket = 0
    - bucket = 0 + 100 = 100

"M" (3º)
- num = 1000
- counter_rep = 1
- condition:
    - 100 (last) < 1000 (actual)
- exec:
    - bucket = 1000 - 100 = 900
    - sum = 1000 + 900 (bucket) = 1900
    - bucket = 0

"X" (4º)
- num = 10
- counter_rep = 1
- condition:
    - 1000 (last) > 10 (actual)
- exec:
    - sum = 1900 + 0 (bucket) = 1900
    - bucket = 0
    - bucket = 0 + 10 = 10

"C" (5º)
- num = 100
- counter_rep = 1
- condition:
    - 10 (last) < 100 (actual)
- exec:
    - bucket = 100 - 10 = 90
    - sum = 1900 + 90 (bucket) = 1990
    - bucket = 0

"I" (6º)
- num = 1
- counter_rep = 1
- condition:
    - 100 (last) > 1 (actual)
- exec:
    - sum = 1990 + 0 (bucket) = 1990
    - bucket = 0
    - bucket = 0 + 1 = 1

"V" (7º)
- num = 5
- counter_rep = 1
- condition:
    - 1 (last) < 5 (actual)
- exec:
    - bucket = 5 (actual) - 1 (bucket) = 4
    - sum = 1990 (sum) + 4 (bucket) = 1994
    - bucket = 0
- condition:
    - index == c.len()
- exec:
    - sum = 1994 (sum) + 0 (bucket) = 1994


