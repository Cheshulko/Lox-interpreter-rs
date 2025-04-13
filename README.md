# Lox interpreter

> ### ⚠️ Disclaimer
> This project is currently in **development** and is **not production-ready**.
> Although it draws inspiration from the Lox language as described in [Crafting Interpreters](https://craftinginterpreters.com), it **does not exactly replicate** the original book's implementation. This version may include deviations, bugs, incomplete features, or experimental code.

## 📘 What is Lox?
Lox is a dynamically-typed, interpreted programming language created specifically for teaching purposes in the book [Crafting Interpreters](https://craftinginterpreters.com) by [Robert Nystrom](https://journal.stuffwithstuff.com). It combines a simple, readable syntax with enough expressive power to support real programming constructs like **functions**, **classes**, and **closures**. It supports first-class functions, lexical scoping, dynamic typing, and a small standard library. 
Its object model supports classes, inheritance, and method dispatch. Lox also emphasizes simplicity in its grammar and implementation, making it easy to experiment with and extend.

## 🦀 What is this project?
This project is a Rust implementation of the Lox programming language interpreter, inspired by Robert Nystrom's Crafting Interpreters. It serves as both a learning exercise and a foundation for further experimentation in language design and implementation. 


## 🚀 Features
The interpreter encompasses several core components:
* **Scanning** (Lexical Analysis): Transforms raw source code into a sequence of tokens, identifying keywords, literals, and symbols.​
* **Parsing**: Processes tokens to construct an Abstract Syntax Tree (AST) that represents the program's grammatical structure.​
* **Evaluation**: Traverses the AST to execute expressions and statements, producing the desired program behavior.
* **Error Handling**: Manages syntax and runtime errors gracefully, providing meaningful feedback to the user.​
* **Environment and Scope Management**: Handles variable declarations, scoping rules, and maintains state across different blocks and functions.​

## 🧑‍💻 Getting Started
### Prerequisites
* Rust toolchain (cargo, rustc)

#### Building the Project
```bash
cargo build --release
```
#### Running a Lox Program
```bash
cargo run -- run path/to/your/program.lox
```
<details>
  <summary>Example</summary>

  ```js
// This program creates a function that returns another function
// and uses it to filter a list of numbers
fun makeFilter(min) {
    fun filter(n) {
      if (n < min) {
        return false;
      }

      return true;
    }

    return filter;
}

fun applyToNumbers(f, count) {
    var n = 0;
    while (n < count) {
      if (f(n)) {
        print n;
      }
      
      n = n + 1;
    }
}

var greaterThanX = makeFilter(21);
var greaterThanY = makeFilter(45);

print "Numbers >= 21:";
applyToNumbers(greaterThanX, 21 + 4);

print "Numbers >= 45:";
applyToNumbers(greaterThanY, 45 + 4);
```
> Numbers >= 21: <br>
> 21  <br>
> 22  <br>
> 23  <br>
> 24  <br>
> Numbers >= 45:  <br>
> 45  <br>
> 46  <br>
> 47  <br>
> 48  <br>
</details>


#### Tokenize a Lox Program
```bash
cargo run -- tokenize path/to/program.lox
```
<details>
  <summary>Example</summary>

```js
var a = "hello";
```
> VAR var null <br>
> IDENTIFIER a null <br>
> EQUAL = null <br>
> STRING "hello" hello <br>
> SEMICOLON ; null <br>
> EOF  null <br>

```js
var greeting = "Hello"
if (greeting == "Hello") {
    return true
} else {
    return false
}
```
> VAR var null <br>
> IDENTIFIER greeting null <br>
> EQUAL = null <br>
> STRING "Hello" Hello <br>
> IF if null <br>
> LEFT_PAREN ( null <br>
> IDENTIFIER greeting null <br>
> EQUAL_EQUAL == null <br>
> STRING "Hello" Hello <br>
> RIGHT_PAREN ) null <br>
> LEFT_BRACE { null <br>
> RETURN return null <br>
> TRUE true null <br>
> RIGHT_BRACE } null <br>
> ELSE else null <br>
> LEFT_BRACE { null <br>
> RETURN return null <br>
> FALSE false null <br>
> RIGHT_BRACE } null <br>
> EOF  null <br>
</details>

#### Parse a Lox Program (AST)
```bash
cargo run -- parse path/to/program.lox
```
<details>
  <summary>Example</summary>

```js
85 - 96 * 47 - 58
```
> (- (- 85.0 (* 96.0 47.0)) 58.0)
```js
(23 != 60) == ((-24 + 57) >= (14 * 40))
```
> (>= (group (- 83.0 62.0)) (- (group (+ (/ 66.0 33.0) 86.0))))
</details>

#### Evaluate a Lox Program
```bash
cargo run -- evaluate path/to/program.lox
```
<details>
  <summary>Example</summary>

```js
(83 - 62) >= -(66 / 33 + 86)
```
> true
```js
10 + 35 - (-(81 - 80))
```
> 46   
</details>

# 📄 Feature Examples

#### Print Statement
```js
print "Hello, Lox!";
```
> Hello, Lox!
#### Variable Declaration and Assignment
```js
var name = "Lox";
print name;
name = "Crafting";
print name;
```
> Lox <br>
> Crafting
#### Arithmetic Expressions
```js
print 3 + 2 * (4 - 1);
```
> 9
#### Control Flow: *if* Statement
```js
var x = 10;
if (x > 5) {
  print "Greater";
} else {
  print "Smaller";
}
```
> Greater
#### Control Flow: While Loop
```js
var i = 0;
while (i < 3) {
  print i;
  i = i + 1;
}
```
> 0 <br>
> 1 <br>
> 2
#### Functions
```js
fun square(n) {
  return n * n;
}
print square(5);
```
> 25
#### Recursion
```js
fun factorial(n) {
  if (n <= 1) return 1;
  return n * factorial(n - 1);
}
print factorial(4);
```
> 24
#### Lexical Scope / Closures
```js
fun makeCounter() {
  var count = 0;
  fun inc() {
    count = count + 1;
    return count;
  }
  return inc;
}
var counter = makeCounter();
print counter();
print counter();
```
> 1 <br>
> 2
#### Classes and Instances
```js
class Greeter {
  greet() {
    print "Hello from object!";
  }
}
var g = Greeter();
g.greet();
```
> Hello from object!
#### *this* Keyword
```js
class Counter {
  init() {
    this.count = 0;
  }

  inc() {
    this.count = this.count + 1;
    print this.count;
  }
}
var c = Counter();
c.inc();
c.inc();
```
> 1 <br>
> 2
#### Inheritance and *super*
```js
class A {
  speak() {
    print "A";
  }
}

class B < A {
  speak() {
    super.speak();
    print "B";
  }
}

var obj = B();
obj.speak();
```
> A <br>
> B

# 📜 License
MIT
