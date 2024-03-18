## Packages and Crates ##

Definitions:

* **Crate**
    * The smallest unit of code that the Rust compiler considers at a time.
    * Contains one or more modules (which may be defined in multiple files)
    * Comes in one of two forms
        * *Binary Crate*:
            * Program that compiles to an executable
        * *Library Crate*:
            * Define functionality to be shared with multiple projects
            * Typically, this is what's meant when programmers use the term
              *crate*.
    * The *Crate Root* is the source file the compiler starts from and
      contains the root module of the crate.

* **Package**:
    * A bundle of one or more crates, providing a set pf functionality
    * The package must contain the *Cargo.toml* file, which defines how to
      build the crates in the package.
    * Also contains a library crate that the binary crate depends on.

---

### Creating a Package ###

To create a package, we execute the following command

```powershell
cargo new project_name
```

This generates the following items

```
/project_name
:   ├── Cargo.toml
:   └── /src
:   :   └── main.rs
```

Note: Cargo.toml does not mention /src/main.rs. This is because Cargo follows 
the convention that the *crate root* of the package is:
* ***src/main.rs*** for a binary package<br>or
* ***src/lib.rs*** for a library package

If the package contains both *src/main.rs* and *src/lib.rs*, then Cargo will
understand that it contains both binary and library crates and build both.
Both will have the same name as the package.

Additionally, you can include multiple binary crates by including them in
the *src/bin* directory.

---

Note: The book provides a "cheat sheet" at this point. It has been recreated as:<br>[02cs-modules-cheat-sheet.md](./02cs-modules-cheat-sheet.md)

---
