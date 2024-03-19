## Compiling and Running a Rust Program ##

***Compiling and executing are separate steps!***

You may be used to working with scripting languages or other
languages that utilize just-in-time compilation.

Rust is an ahead-of-time compiled language, so it is 
necessary to compile an executable file before it can be
run.

---

### Compiling a Rust Program ###

There are two main mechanisms to compile a program:

* Use the Rust compiler directly with the following command:
  <br>```rustc main.rs```
* If you have created a Cargo project, use the following 
  command:<br>```cargo build```

Note: The files created differ depending on the environment.

* On Windows, the following files are generated:
    * main.exe
    * main.pdb
* On Linux or MacOS, the following file is generated:
    * main

---

### Running a Rust Program ###

After compiling the program, you need to run the exe
that was generated during compilation.

* On Windows, run the following command:<br>```.\main.exe```
* On Linux or MacOS, run the following command:<br>
  ```./main```

---

### Compile and Run with Cargo ###

If you created a Cargo project, use the following command to
perform both the compile and run steps:<br>```cargo run```

---
