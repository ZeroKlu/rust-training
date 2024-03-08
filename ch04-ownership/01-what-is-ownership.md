## What is Ownership ##

There are a number of approaches to memory management:

* In languages like C and C++, the programmer must explicitly manage memory
  in the code.
* In others, like Java and .NET, the language implements garbage collection
  to clean up no-longer-needed resources.
* In Rust, memory is managed through ownership rules, which are implemented
  at compile-time and ensure that the code cannot build unless the rules
  are followed.

Ownership is unique to Rust and provides the mechanism to make the code
memory-safe without the implementation of a garbage collector.

---

### Stack vs. Heap ###

In Rust, the behavior of the language depends on whether a value is on the
stack or the heap.

#### Definitions ####

* **Stack**:
    * Relatively small area of memory
    * LIFO: last-in-first-out
        * Stores (push) values in the order they are created
        * Removes (pop) values in the opposite order
    * All data on the stack must have a known, static memory size
    * Fast storage, because (given known memory size) the next location is
      always the top of the stack
* **Heap**:
    * Large, less organized memory area
    * Data stored wherever the memory allocator locates a sufficient-sized
      block of available RAM
    * Memory allocator returns a pointer to a heap location (the pointer is 
      stored on the stack).
    * Slower access because the pointer must be retrieved from the stack and
      then followed to the memory location on the heap.

During a function call, all values passed to the function (including heap
pointers) and all locally-scoped variables are pushed onto the stack, and
when the function returns, those values are popped off.

Managing the heap is more complicated, and ownership provides the rule set
by which heap memory is managed.

---

### Fundamental Ownership Rules ###

1. Each value in Rust has an *owner*
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

---
