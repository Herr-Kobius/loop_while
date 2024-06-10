<h1 align="center">
  <br>
  <br>
  LOOP/WHILE Interpreter
  <br>
</h1>

<h4 align="center">This is an interpreter for the recursive language LOOP, which is based on the counter-machine model.
<br>
<br>
<p align="center">
  <a href="https://www.rust-lang.org">
    <img src="https://img.shields.io/badge/Rust-1.74.0-red.svg?logo=rust">
  </a>
  <a href="https://docs.rs/crate/serde_json/latest">
    <img src="https://img.shields.io/badge/Serder JSON-1.0.48-pink.svg?logo=serder">
  </a>
</p>


## Description
### LOOP

The LOOP language utilizes a small set of imperative instructions, allowing for the creation of simple programs that can be inductively proven.

The LOOP language has two types of statements:
- **Assignments:** Writing a variable with addition/subtraction
- **LOOP:** A counting loop, also known as for loop

A program, denoted as $prog$, will adhere to the following syntax:

$$
\begin{align}
   prog &\to [stat]^{*}\\   
   stat &\to assi|loop\\ 
   assi &\to var:=var\pm num;\\\
   loop &\to \text{LOOP}\:var\:\text{DO}\:stat\:\text{END}\\
   var  &\to var\_{prefix}\circ num\\
   num  &\to [0..9]^{*}\\
   var\_{prefix} &\to a..z
\end{align}
$$

A program will receive a list of natural numbers as inputs.
If a variable $var$ is not initialized, it will be assigned the value $0$.
A program, denoted as $prog$, calculates the function
$$\mathbb{N}^k\to\mathbb{N} (k \in \mathbb{N})$$

All keywords, symbols, and variable prefixes can be customized in a JSON file (syntax.json).

### WHILE

WHILE-Programs are LOOP-Programs that also have the option of one additional instruction type:

- **WHILE:** A loop with a condition

The syntax for the while loop is as follows:
$$
\begin{align}
    stat &\to assi|loop|while\\ 
    while &\to \text{WHILE }\:var!=0\:\text{ DO}\:stat\:\text{END}
\end{align}
$$

A WHILE-Program requires the ending *".while"*, the ending *".loop"* is sufficient for a LOOP-Program.
## References 

[LOOP Language - Wikipedia](https://en.wikipedia.org/wiki/LOOP_(programming_language))

[Serder JSON](https://docs.rs/serde_json/latest/serde_json/)

## Credits

The Interpreter’s precise configuration is based on the lecture:

<h4 align="center">
  Theoretische Informatik und Logik<br><a href="https://iccl.inf.tu-dresden.de/web/Wissensbasierte_Systeme/en">
   Knowledge-Based Systems - TU Dresden
  </a>
  <br>
</h4>
<br>

This software uses the following packages:

[Serder JSON](https://docs.rs/crate/serde_json/latest)

## License  

[Attribution-NonCommercial-ShareAlike 4.0 International](/LICENSE.txt)



