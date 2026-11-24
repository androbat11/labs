# Interactive Workbook: Array Indexing, Memory Math & Systems Drills

This workbook is designed to test and solidify your mental model of physical memory, address calculation, unit conversion, and hardware cache behavior.

You can edit this file directly to write your answers under each `> **Your Answer:**` prompt, or answer them in our chat as we work through them!

---

## Part 1: Conceptual & Socratic Understanding

### Question 1.1: The Dimensional Units
In the formula $\text{Offset} = i \times S$:
* What physical unit does the variable $i$ (index) carry?
* What physical unit does the variable $S$ carry?
* Show the dimensional cancellation that proves the final result is in **bytes**.

> **Your Answer:**
> 

---

### Question 1.2: The Meaning of Zero-Based Indexing
A colleague asks you: *"Why does computer programming start counting at 0 instead of 1? It's counter-intuitive to normal human counting!"*

Using the concept of **offsets** and the formula $\text{Address}(i) = \text{Base} + (i \times S)$, explain why starting at `0` is mathematically natural for computer memory.

> **Your Answer:**
> 

---

### Question 1.3: Address vs. Value (Dereferencing)
Explain the difference between these two concepts:
1. The **Address** calculated by $\text{Base} + (i \times S)$.
2. The **Value** obtained by `arr[i]`.
*(Feel free to use the Locker Analogy or the assembly instruction `mov eax, [rbx + rdi * 4]` to illustrate).*

> **Your Answer:**
> 

---

## Part 2: Manual Address Calculation Drills

Use the formula: $\text{Address}(i) = \text{Base Address} + (i \times S)$

*Reference Table of Sizes:*
* `u8` / `bool`: $S = 1$ byte
* `i16`: $S = 2$ bytes
* `i32` (or standard integer): $S = 4$ bytes
* `i64` / 64-bit pointer: $S = 8$ bytes

---

### Drill 2.1: Simple Decimal Math
An array of 32-bit integers (`i32`, where $S = 4$ bytes) begins at physical byte address **`1000`** (in decimal).
1. What is the address of `arr[0]`?
2. What is the address of `arr[5]`?
3. What is the byte offset required to reach `arr[12]`?

> **Your Work & Answer:**
> 1. arr[0] = 
> 2. arr[5] = 
> 3. Offset for arr[12] = 

---

### Drill 2.2: Working with Hexadecimal Addresses
Memory addresses are almost always printed in hexadecimal (base 16).
Recall that in hex: `0x0 + 0x4 = 0x4`, `0x8 + 0x4 = 0xC` (12), `0xC + 0x4 = 0x10` (16).

An array of 64-bit floating point numbers (`f64`, where $S = 8$ bytes) is allocated starting at **`0x2000`**.
1. What is the memory address of `arr[0]`?
2. What is the memory address of `arr[1]`?
3. What is the memory address of `arr[3]`? (Hint: $3 \times 8 = 24$ in decimal. What is $24$ in hex?)

> **Your Work & Answer:**
> 1. arr[0] = 
> 2. arr[1] = 
> 3. arr[3] = 

---

### Drill 2.3: Different Data Types
Suppose an array of 8-bit bytes (`u8`, $S = 1$) starts at address **`0x5000`**.
What is the memory address of `arr[42]`?

> **Your Work & Answer:**
> 

---

## Part 3: Boundaries, Footprint & Sentinel Limits

Suppose you allocate an array in Rust:
```rust
let scores: [i32; 100] = [0; 100]; // 100 integers (i32, S = 4)
```
The base address is **`0x4000`**.

### Problem 3.1:
What is the **Max Valid Index** of this array?

> **Your Answer:**
> 

### Problem 3.2:
What is the exact starting address of the **last valid element** in the array?
*(Show your formula and calculation).*

> **Your Work & Answer:**
> 

### Problem 3.3:
What is the **Total Memory Footprint** (in bytes) of this array?

> **Your Work & Answer:**
> 

### Problem 3.4:
In systems languages like C++ and Rust, iterators often use a "one-past-the-end" sentinel pointer (`arr.end()`).
What is the address of the byte immediately following this entire array?

> **Your Work & Answer:**
> 

---

## Part 4: "Bug Detective" (Diagnostic Scenarios)

### Bug Scenario 4.1: The Addition Catastrophe
Junior engineer Bob writes a custom memory allocator in C. For his lookup function, he implements the formula as:
$$\text{Target} = \text{Base} + (i + S)$$
He tests it on an array of `i32` ($S = 4$) with $\text{Base} = 1000$.
1. If Bob queries index $i = 0$, what address does he get? What *should* he have gotten?
2. If Bob queries index $i = 2$, what address does he get? Where in memory does that actually point?

> **Your Analysis:**
> 

---

### Bug Scenario 4.2: The Stride Mismatch
Alice has an array of 64-bit pointers (`usize`, $S = 8$ bytes). However, in her indexing assembly logic, she accidentally hardcoded the scale factor as `4` instead of `8`:
$$\text{Calculated Address} = \text{Base} + (i \times 4)$$
If she attempts to read index $i = 2$:
* What byte offset does her code compute?
* What was the expected byte offset?
* What kind of data will she read when she dereferences that address?

> **Your Analysis:**
> 

---

## Part 5: Hardware Cache Line Challenge

Modern x86 and ARM CPUs load data from RAM into L1 Cache in **64-byte chunks** called **Cache Lines**.

```
Memory Bus: [ Reads 64 Contiguous Bytes in 1 cycle ]
```

### Scenario 5.1:
Suppose you iterate sequentially through an array of 32-bit integers (`i32`, $S = 4$ bytes):
`for i in 0..100 { sum += arr[i]; }`

1. How many `i32` elements fit inside a single 64-byte cache line?
2. When reading `arr[0]`, the CPU experiences a **Cache Miss** (it must fetch 64 bytes from RAM).
   For how many subsequent reads (`arr[1]`, `arr[2]`, etc.) will the CPU get an ultra-fast **Cache Hit** before it needs to fetch another cache line?
3. What is the cache miss rate for this loop (e.g. 1 miss every $K$ accesses)?

> **Your Work & Answer:**
> 1. Elements per cache line = 
> 2. Consecutive cache hits = 
> 3. Miss rate = 
