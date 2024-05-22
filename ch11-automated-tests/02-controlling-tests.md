## Controlling How Tests Are Run ##

When you execute ```cargo test```, cargo compiles a test binary. It then
(by default) runs all tests in parallel and captures any output (does not
display output).

You can add command-line arguments to control how tests are run. There are
two separate types of command-line arguments: ones that control
```cargo test``` itself and ones that are passed to the binary. These are 
separated by a double-dash [```--```].

```
cargo test [cargo args] -- [binary args]
```

To access a list of the cargo arguments, run this:

```
cargo test --help
```

To access a list of the binary arguments, run this:

```
cargo test -- --help
```
