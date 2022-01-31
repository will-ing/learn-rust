# Iterators

Iterator - Allow you to iterate regardless of how they are stored. They contain the logic to iterate over different data structures.

They are inherently *lazy*, they have no effect until you call the methods that consume them.

Type of Iterators:

```rs
iter() //returns immutable references
iter_mut() //returns mutable references
into_iter() // Own types
```

## Methods:

next() - this is a consuming adapter, this uses up the iterator. Iterates through the item by repeatedly calling next.

collect() - consumes the iterator and collects the resulting values.

sum() - takes ownership and returns the total of the structure.

***
## Iterator Adapters

Allow you to change iterators into different kind of iterators

```rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

map() - iterates through data structure and executes some action, similar to a for loop.

filter() - iterates and returns values that meet the criteria. 

references: \
https://doc.rust-lang.org/book/ch13-02-iterators.html

