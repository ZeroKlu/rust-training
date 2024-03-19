## Using Cargo to Manage a Project ##

Cargo is the build system and package manager for Rust. It
handles a lot of the heavy lifting for you, including:

* Building the code
* Downloading libraries
* Building dependencies

You can validate the installation of cargo by running the
following command:<br>```cargo --version```

---

### Creating a Project with Cargo ###

To create a project, run the following command:<br>
```cargo new project_name```

This generates a project structured like this:

```
/project_name
├── /.git
├── Cargo.toml
└── /src
:   └── main.rs
```

* **Cargo.toml** contains the description of the project used
  during the build process.
* **/src/main.rs** contains the Rust code file with the 
  ```main()``` function.
* **/.git** contains a Git repository configuration for the
  project.

---

### Skipping .git ###

If you created the new Cargo project inside a directory that
already contains a Git repository, the .git folder will not be
created.

If you want to explicitly exclude creating a Git repository,
you can add the 'vcs' (version control system) switch to the
command, like this:<br>```cargo new project_name --vcs none```

---

### Cargo.toml ###

The Cargo.toml ([Tom's Obvious, MinimalLanguage](https://toml.io))
file describes the package to be created by Cargo's build process.

```toml
[package]
name = "project_name"
version = "0.1.0"
edition = "2021"

[dependencies]
```
The ```[package]``` section defines the project itself.

The ```[dependencies]``` section is used to identify libraries
that need to be downloaded and included in the build.

---
