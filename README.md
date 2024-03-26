# README #

## Rust Training ##

### Textbook Info ###

|            |                                                                                                          |
|------------|----------------------------------------------------------------------------------------------------------|
| Title:     | [The Rust Programming Language, 2nd Edition](https://nostarch.com/rust-programming-language-2nd-edition) |
| Author:    | Steve Klabnik and Carol Nichols                                                                          |
| ISBN:      | [9781718503106](https://isbnsearch.org/isbn/9781718503106)                                               |
| Publisher: | [No Starch Press](https://nostarch.com/)                                                                 |
|||

Note: The textbook is also available free online:
[doc.rust-lang.org/book](https://doc.rust-lang.org/book)

Brown University also has a version with quizzes
and additional notes/diagrams:
[rust-book.cs.brown.edu](https://rust-book.cs.brown.edu)<br>
Diagrams in the markdown notes are all from this site

---
### What is this repository for? ###

* Basic training on Rust Programming Language fundamentals

---

### Setup/Requirements ###

* Install Rustup (Rust Language Components)
    ```powershell
    winget install Rustlang.Rustup
    ```

* Install MS Build Tools (if Visual Studio is not already installed)
    ```powershell
    winget install Microsoft.VisualStudio.2022.BuildTools
    ```

* Modify Build Tools and/or Visual Studio (using Visual Studio Installer) to include:
    * C/C++ Desktop Development

* Install the
  [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  Visual Studio Code Extension
    * This extension is required in order to work with Rust in Visual Studio Code

* Optionally install the
  [code-runner](https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner)
  Visual Studio Code Extension
    * This extension simplifies running/testing code in a number of languages (including Rust)

---

### Known Conflicts/Compatibility Notes ###

* rust-analyzer will show a workspace load error when working in
  this repository. This is normal and will not cause problems working
  with, building, or executing the code.

---

### Documentation ###

* [Setting Up Your Workstation for Development Training.pdf](./additional-files/Setting%20Up%20Your%20Workstation%20for%20Development%20Training.pdf)
* [Setting Up Your Workstation for Rust Training.pdf](./additional-files/Setting%20Up%20Your%20Workstation%20for%20Rust%20Training.pdf)

---

### Who do I talk to? ###

* Any questions can be addressed to the following
    * [✉ Zero Klu](mailto:zeroklu@myself.com?subject=Rust%20Training&body=Question%20about%20your%20Rust%20Training%20repository%20on%20GitHub:)
