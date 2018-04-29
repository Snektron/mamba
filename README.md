# Mamba

Mamba is a simple tool for interpreting turing machines, as well as generating a latex
tabular turing machines.

## Installation

To install Mamba, clone it into a directory and run `cargo install`:
```
git clone https://github.com/Snektron/mamba
cargo install
```
Sadly it takes about 10 minutes to compile.

## Turing machine delta function

The turing machine's delta function is read from a file and has the following structure:

* Any `<movement>` represent the movement of the head, and consist of  `<`, `>` or `!` to respectively move left, right or not at all.

* Any `<state>` represents a state, which can consist of any combination alphanumeric characters.

* Any `<symbol>` is a tape symbol, which can consist of any combination alphanumeric characters.
    The special symbol `_` represents the blanco symbol and cannot appear in the alphabet, as it is implicitly added.

* Input alphabet, accept state and reject state. This is always defined at the top of the file and in this order.
    ```
    alphabet: 0, 1;
    accept: qY;
    reject: qN;
    ```

* Simple transitions. These represent a single transition and have the format `<from-state> <read-symbol> -> <to-state> <write-symbol> <movement>`.
    ```
    q0 0 -> q1 1 >
    ```

* Block transitions. have the format `<from-state> '{' (<read-symbol> -> <to-state> <write-symbol> <movement>)+ '}'`
    and are for defining a group transitions on a state. Not all transitions for a state have to be defined in the same block.
    ```
    q0 {
        0 -> q1 1 >
        1 -> q0 _ <
        _ -> q2 0 !
    }
    ```

* The special symbol `*` can be used a wildcard to automatically infer undefined rules.
    ```
    q0 {
        0 -> q0 1 >
        * -> q1 * <
    }
    q1 * -> 1 <
    ```

## Execution

To execute a turing machine run `mamba run <source>`. The initial tape can either be specified using
`-i <input>`, via a file with `-f <file>`. The input isnt specified via `-i` or `-f` its read from stdin.
A debug `-d` flag can be given to print the tape after each step. 

## Latex generation

To generate a latex representation of the alphabet and the delta function, run `mamba latex <source>`.
