## Memory and Allocation ##

* Memory is requested from the memory allocator during runtime
* Therefore it has to be returned to the memory allocator when we're done
  with it.

The request for memory takes place within the implementation of
```String::from()```

Returning the memory is different.

* In garbage-collected languages, the GC keeps track of in-use allocations
  and cleans up when items are no longer in use.
* In most other languages, it's the responsibility if the programmer to
  return (deallocate) memory. (think C's ```malloc() ... free()``` model)
    * In a language like this, there needs to be a one-for-one pairing of one
      free for every allocate.
    * This is enough of a problem that utilities have been written just to
      scan code for missing free calls to prevent issues like:
        * Free too late: memory is wasted
        * Free too early: invalid reference
        * Free twice: error! trying to release already released memory

* In Rust, memory is returned as soon as it goes out of scope, eliminating
  both the need for a garbage collector and the need for explicit calls to
  free memory.

```rust
{
    let s = String::from("hello"); // from this point forward, s is valid
    // do things with s
}                   // after the block (scope) ends, s is no longer valid 
```

Under the covers, Rust calls a special function ```drop``` when the value
goes out of scope.

Note: This mimics the C++ RAII (Resource Acquisition Is Initialization)
pattern.

---
