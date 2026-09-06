# Implementation Plan: Lambda Calculus with Primitives

## 🎯 The Goal
To upgrade the existing Lambda Calculus evaluator into a "Micro-Lisp" or "Lambda Calculus with Primitives." By adding native numbers, booleans, and basic operations, the language will transition from an academic exercise (Church Encodings) into a human-readable functional programming language, while keeping the core lambda calculus engine intact.

---

## 🛠️ Step 1: Update the AST (`src/parser.rs`)
We need to expand the `Term` enum to support our new native data types and operations.

**Changes:**
1. Add `Int(i32)` and `Bool(bool)` to the `Term` enum.
2. Add `BinaryOp(Operator, Box<Term>, Box<Term>)` for math operations.
3. Add `IfElse(Box<Term>, Box<Term>, Box<Term>)` for control flow.
4. Define a simple `Operator` enum (e.g., `Add`, `Sub`, `Mul`, `Eq`).
5. Update the `fmt::Display` implementation for `Term` to print these new variants cleanly (e.g., printing `5` instead of an AST representation, and optimizing parenthesis usage).

---

## 📝 Step 2: Update the Lexer (`src/lexer.rs`)
The lexer needs to recognize the new syntax and generate the corresponding tokens before they reach the parser.

**Changes:**
1. Add new token variants to the `Token` enum: `Number(i32)`, `TrueTok`, `FalseTok`, `Plus`, `Minus`, `Asterisk`, `Equals`, `IfTok`, `ThenTok`, `ElseTok`.
2. Modify the `tokenize` function to regex-match or parse contiguous digits into a `Number(i32)` token.
3. Add string matching for keywords (`true`, `false`, `if`, `then`, `else`).
4. Add character matching for the operators (`+`, `-`, `*`, `==`).

---

## 🏗️ Step 3: Update the Parser (`src/parser.rs`)
The parser needs to translate the new tokens into our expanded `Term` AST.

**Changes:**
1. **Primitives:** When `parse_term` encounters a `Token::Number` or boolean token, it should immediately return the `Term::Int` or `Term::Bool` AST node.
2. **Binary Operations:** Implement logic to parse expressions like `t1 + t2`. (Note: To keep things simple at first, you can use prefix notation like `(+ 1 2)` to avoid dealing with complex operator precedence, or implement standard infix parsing if you feel adventurous).
3. **If/Else:** When the parser hits `Token::IfTok`, it should parse the condition, look for `Token::ThenTok`, parse the true-branch, look for `Token::ElseTok`, and parse the false-branch.

---

## ⚙️ Step 4: Update the Reducers (`src/reducer.rs`)
The evaluation engine must know how to handle the new AST nodes. This is the most satisfying part, as you hand off the actual computation to Rust!

**Changes:**
1. **BinaryOp Reduction:** If the reducer encounters a `Term::BinaryOp`, it should first reduce both the left and right sides. If both reduce down to `Term::Int`, it performs the actual Rust math (e.g., `left_val + right_val`) and returns a new `Term::Int` with the result.
2. **If/Else Reduction:** If the reducer encounters `Term::IfElse(cond, true_branch, false_branch)`, it first reduces `cond`. 
   - If `cond` reduces to `Term::Bool(true)`, it returns `true_branch`.
   - If `cond` reduces to `Term::Bool(false)`, it returns `false_branch`.
3. **Variable Substitution:** Update the `fv` (free variables) and `substitute` functions to simply ignore `Term::Int` and `Term::Bool`, since primitives don't have variables to replace. For `BinaryOp` and `IfElse`, the substitution should just recursively pass down into their child nodes.

---

## 🚀 Step 5: Test in the REPL (`src/main.rs`)
Fire up the newly created REPL and test your human-readable inputs!

**Test Cases to run:**
1. `let x = 5 in x + 10`  *(Should output: 15)*
2. `if true then 1 else 0` *(Should output: 1)*
3. `(\x. x * 2) 21` *(Should output: 42)*

---
*Ready for your next session! Good luck building your Micro-Language!*
