# Eval

## Eval is a CLI REPL math expression evaluator
The program was written to study various aspects of interpreter and compiler design and development. Such as lexical analysis, parsing to an AST (Abstract Syntax Tree), and evaluation by the REPL interpreter. The program itself is written in rust.

#### Usage
Running the program will enter the repl environment where various match expressions can be evaluated.


##### Example
<pre>
jakesnell eval: cargo r
   Compiling eval v0.1.0 (**PATH**)
    Finished dev [unoptimized + debuginfo] target(s) in 1.07s
     Running `target/debug/eval`

                 _ _
  _____   ____ _| | |
 / _ \ \ / / _` | | |
|  __/\ V / (_| | |_|
 \___| \_/ \__,_|_(_)
 
 welcome to eval repl.
 enter ".help" for help.


>>> -4**3 + 3 / 4 + 3.5548
-59.695198
</pre>