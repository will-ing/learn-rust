# Closures

What are closures?

1. Like functions but anonymous, 
2. they can be stored as variables and passed around. 
3. Can be passed in as input parameters to a function
4. The capture the variables inside the scope in which they're defined

***

* FnOnce - Take ownership of the variable, can't take ownership of the same variable more than once.
* FnMut - Mutabily borrows valuse
* Fn - immutably borrows values.

***

Can use `move` to explicitly take ownership of a variable.

```rs
 let x = vec![1, 2, 3];

 let equal_to_x = move |z| z == x

 let y = vec![1, 2, 3];

 assert!equal_to_x(y));
```