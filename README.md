# alignby

A small command line utility to align lines around a given character or pattern.

## Examples

```
$ cat values.txt
a = 1
bc = 2
def = 3
g = 4
h=5
ij = 6 = 3 + 3

$ cat values.txt | alignby =
a   = 1
bc  = 2
def = 3
g   = 4
h   =5
ij  = 6 = 3 + 3

```

The `--after` flag is useful when the delimiter should stick to left:

```
$ cat definitions.txt
cat: a small animal
tiger: a big cat
a: indefinite article
elephant: an animal with a long nose

$ cat definitions.txt | alignby :
cat     : a small animal
tiger   : a big cat
a       : indefinite article
elephant: an animal with a long nose

$ cat dfinitions.txt | alignby --after :
cat:      a small animal
tiger:    a big cat
a:        indefinite article
elephant: an animal with a long nose
```
