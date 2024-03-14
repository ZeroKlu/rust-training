## Automatic Referencing & Dereferencing ##

In C and C++, you use one of two different operators when calling a method on
a struct:

* If calling on the object directly, you use ```.``` like this:<br>
  ```object.method()```
* If calling on a pointer to the object you use ```->``` like this:<br>
  ```object->method()``` = ```(*object).method()```

In Rust, you don't have to keep track of this distinction, because both use
the ```.``` operator. This is enabled through the system of *automatic
referencing and dereferencing*.

When you call a method with ```object.method()```, Rust automatically adds
```&```, ```&mut```, or ```*``` to ensure that the method call matches its
signature.

So, these two examples are equivalent:

```rust
obj.method(&arg);
// or
(&obj).method(&arg);
```

This works because methods require the ```self``` type as the first argument
in one of three forms:

* ```self```: method is consuming the struct
* ```&self```: method is reading the struct
* ```&mut self```: method is mutating the struct

Borrowing is implicit for method receivers.

---
