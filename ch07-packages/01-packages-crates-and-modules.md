## Managing Growing Projects with Packages, Crates, and Modules ##

As the size and overall functionality of a project grow, it becomes necessary
to reorganize it into multiple modules and multiple files.

Rust provides the *module system* to help handle this organization. This is 
made up of several component ideas, including:

* **Packages**: Cargo feature to build, test, and share crates
* **Crates**: Tree of modules that produces a library or executable
* **Modules**: Control the organization, scope, and privacy of paths
* **Paths**: Item naming system

---
