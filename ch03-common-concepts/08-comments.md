## Comments ##

Rust has three flavors of comments:

* Single- or partial-line comments using ```//```
  ```rust
  // This is a simple comment
  ```
* Multi-line comments using ```//``` on each line
  ```rust
  // This is a multi-line comment
  // It is used when the information to convey does not
  //   fit on a single line
  ```
    * Rust supports block comments ```/* ... */```, 
      but line-comments are preferred
      ```rust
      /* 
       * This style of comment is supported for
       * multi-line, but is not preferred.
       */
      ```
* Documentation comments using ```///```

---

The following example shows various types of comments (from the Rust documentation):

```rust
//! A doc comment that applies to the implicit anonymous module of this crate

pub mod outer_module {

    //!  - Inner line doc
    //!! - Still an inner line doc (but with a bang at the beginning)

    /*!  - Inner block doc */
    /*!! - Still an inner block doc (but with a bang at the beginning) */

    //   - Only a comment
    ///  - Outer line doc (exactly 3 slashes)
    //// - Only a comment

    /*   - Only a comment */
    /**  - Outer block doc (exactly) 2 asterisks */
    /*** - Only a comment */

    pub mod inner_module {}

    pub mod nested_comments {
        /* In Rust /* we can /* nest comments */ */ */

        // All three types of block comments can contain or be nested inside
        // any other type:

        /*   /* */  /** */  /*! */  */
        /*!  /* */  /** */  /*! */  */
        /**  /* */  /** */  /*! */  */
        pub mod dummy_item {}
    }

    pub mod degenerate_cases {
        // empty inner line doc
        //!

        // empty inner block doc
        /*!*/

        // empty line comment
        //

        // empty outer line doc
        ///

        // empty block comment
        /**/

        pub mod dummy_item {}

        // empty 2-asterisk block isn't a doc block, it is a block comment
        /***/

    }

    /* The next one isn't allowed because outer doc comments
       require an item that will receive the doc */

    /// Where is my item?
}
```