# Memory Layout, Address Calculation & Hardware Reality of Arrays

Understanding how an array exists physically in hardware is the single most important foundation for mastering Data Structures and Algorithms. It explains **why** operations have their respective Big-$O$ costs, rather than treating them as arbitrary rules to memorize.

---

## 1. Physical RAM as a Byte-Addressable Street

Computer memory (RAM) is architected as an enormous, linear sequence of **1-byte mailboxes**. Every mailbox holds **8 bits** of data and possesses a unique, sequential numerical address (commonly represented in hexadecimal, e.g., `0x1000`).

```
Address:    0x1000  0x1001  0x1002  0x1003  0x1004  0x1005  0x1006  0x1007
Byte:      [  B0  ][  B1  ][  B2  ][  B3  ][  B4  ][  B5  ][  B6  ][  B7  ]
```

### The Definition of an Array
An array is a collection of elements that satisfy two strict physical constraints:
1. **Contiguous**: Elements are stored in consecutive, unbroken memory addresses.
2. **Homogeneous (Uniform Size)**: Every element occupies the exact same number of bytes ($S$).

---

## 2. Element Size ($S$) and Primitive Types

Different types require different amounts of physical storage:

| Data Type | Rust Type | Typical C/C++ | TypeScript / JS Runtime | Size ($S$ in bytes) |
|---|---|---|---|:---:|
| Boolean / ASCII Byte | `bool`, `u8`, `i8` | `char`, `bool` | Boolean (engine tagged) | **1** |
| 16-bit Integer | `i16`, `u16` | `short` | Small Integer (SMI) | **2** |
| 32-bit Integer / Float | `i32`, `u32`, `f32` | `int`, `float` | Small Integer (SMI) | **4** |
| 64-bit Integer / Float | `i64`, `u64`, `f64` | `long long`, `double` | JS `number` (IEEE 754 float) | **8** |
| Pointer / Memory Address | `usize`, `&T`, `*const T` | `uintptr_t`, `T*` | Object Reference (64-bit) | **8** |

---

## 3. The Address Calculation Formula

Because the elements are contiguous and uniform in size, calculating where any element lives in RAM is a pure algebraic calculation:

$$\text{Address}(i) = \text{Base Address} + (i \times S)$$

Where:
* **$\text{Base Address}$**: The physical RAM location of the very first element (`arr[0]`).
* **$i$ (Index / Offset)**: How many elements to skip from the beginning.
* **$S$ (Stride / Element Size)**: The byte width of each element.

### Visualizing an Array of Four 32-bit Integers (`i32`)
Let `let arr: [i32; 4] = [10, 20, 30, 40];` with $\text{Base Address} = 0x1000$ and $S = 4$:

```
Index:               arr[0]                 arr[1]                 arr[2]                 arr[3]
Value:                 10                     20                     30                     40
Physical Bytes:   [B0 B1 B2 B3]          [B4 B5 B6 B7]          [B8 B9 B10 B11]        [B12 B13 B14 B15]
Start Address:    0x1000                 0x1004                 0x1008                 0x100C
```

### Why Indexing is 0-Based
In computer systems, an **index is not an ordinal ranking ("1st, 2nd, 3rd")**. 
An index is an **offset distance**:
* `arr[0]` means: *"Go to Base Address, and offset by $0 \times S$ bytes."*
  $$\text{Address} = 0x1000 + (0 \times 4) = 0x1000$$
* `arr[1]` means: *"Go to Base Address, and offset by $1 \times S$ bytes."*
  $$\text{Address} = 0x1000 + (1 \times 4) = 0x1004$$
* `arr[2]` means: *"Go to Base Address, and offset by $2 \times S$ bytes."*
  $$\text{Address} = 0x1000 + (2 \times 4) = 0x1008$$

### Maximum Index & Bounds Calculation
For an array containing $N$ elements:
* **Valid Indices**: $0 \le i \le N - 1$
* **Max Index**: $N - 1$
* **Address of the Final Element**:
  $$\text{Address}(N - 1) = \text{Base Address} + ((N - 1) \times S)$$
* **Total Memory Footprint**:
  $$\text{Total Bytes} = N \times S$$
* **Address Immediately Past the Array** (One-past-the-end sentinel):
  $$\text{Past-End Address} = \text{Base Address} + (N \times S)$$

---

## 4. Why Array Lookup is $O(1)$ Constant Time

To access `arr[999_999]` in an array of one million integers, the CPU does **not** scan elements 0 through 999,998.

Instead, the CPU arithmetic logic unit (ALU) computes:
$$\text{Target} = \text{Base} + (999\,999 \times 4)$$
This requires **one multiplication** and **one addition**, which executes in a single CPU cycle ($\approx 0.3$ nanoseconds). Accessing the 1st element or the 1,000,000th element takes the exact same amount of time: **$O(1)$**.

---

## 5. CPU Cache Lines & Spatial Locality

CPUs do not read RAM byte-by-byte. Memory transfers between RAM and the CPU occur in chunks called **Cache Lines** (typically **64 bytes** wide).

```
RAM Transfer: [ 64 Contiguous Bytes fetched in one memory cycle ]
              ├── arr[0] ──┤├── arr[1] ──┤ ... ├── arr[15] ──┤
```

* When your code reads `arr[0]`, the CPU's memory controller automatically loads the entire 64-byte block into the ultra-fast **L1 Data Cache**.
* In an `i32` array ($4$ bytes per element), one 64-byte line holds **16 consecutive integers**.
* When your loop reaches `arr[1]`, `arr[2]`, ..., `arr[15]`, the CPU gets a **Cache Hit** (access latency $< 1$ ns) rather than waiting for main RAM ($50-100$ ns).
* This property is called **Spatial Locality**, and it is the primary reason contiguous arrays outperform linked lists or node trees by orders of magnitude on modern hardware.

---

## 6. Language Runtime Comparison: Rust vs. TypeScript / Bun

### Rust (Systems Reality)
```rust
// 1. Fixed Array: Allocated directly on the STACK
let fixed: [i32; 4] = [10, 20, 30, 40]; // 16 contiguous bytes on stack frame

// 2. Dynamic Vector: Heap buffer managed by a 3-word stack descriptor
let mut dynamic: Vec<i32> = vec![10, 20, 30, 40];
```

In 64-bit Rust, `Vec<T>` on the stack is exactly 24 bytes:
1. `ptr`: 8-byte pointer to heap memory where the contiguous buffer lives.
2. `capacity`: 8-byte integer (how many elements the buffer can hold before reallocating).
3. `len`: 8-byte integer (how many elements currently exist).

```
STACK                                HEAP (Contiguous Buffer)
[ ptr: 0x5000 | cap: 4 | len: 4 ] -> 0x5000: [ 10 | 20 | 30 | 40 ]
```

### TypeScript / JavaScript (Bun / V8 / JSC Reality)
In JavaScript engines (like JavaScriptCore in Bun or V8 in Node):
* Arrays are dynamic objects. The engine optimizes sequential numeric arrays into **Packed SMI (Small Integer)** or **Packed Double** internal C++ arrays.
* However, if you create sparse entries or mix data types:
  ```ts
  const arr = [1, 2, 3];
  arr[10000] = 99; // Introduces holes!
  ```
  The engine de-optimizes the array into **Dictionary Mode** (Hash Table lookup). Indexing degrades from $O(1)$ direct address arithmetic to hash table lookups with collisions and GC overhead.

---

## 7. Key Takeaways
1. **$O(1)$ Access**: Comes directly from the mathematical formula $\text{Base} + (i \times S)$.
2. **Offset Semantics**: Index $0$ is an offset of zero steps from base.
3. **Contiguity**: Enables hardware cache line prefetching (Spatial Locality).
4. **Insertion/Deletion Cost**: Inserting at index $0$ requires shifting all $N$ subsequent elements by $S$ bytes to preserve contiguity, making it **$O(N)$**.

---

## 8. Recommended Reading Materials

To deepen your systems-level intuition regarding memory, caching, and array representations, here are the highest-quality references:

### 1. The Gold Standard Systems Textbook
* **"Computer Systems: A Programmer's Perspective" (CS:APP)** by Randal E. Bryant & David R. O'Hallaron
  * **Section 3.8: Array Allocation and Access**: Explains exactly how machine and assembly instructions compute array offsets for 1D, multi-dimensional, and variable-sized arrays.
  * **Chapter 6: The Memory Hierarchy**: Breaks down L1/L2/L3 CPU cache architectures, cache lines (64 bytes), and cache-friendly code.

### 2. Free High-Quality Online Books & Papers
* **"Operating Systems: Three Easy Pieces" (OSTEP)** by Remzi Arpaci-Dusseau and Andrea Arpaci-Dusseau
  * Free online at: [ostep.org](https://pages.cs.wisc.edu/~remzi/OSTEP/)
  * Focus on the **Virtual Memory** section (address translation, page tables, physical vs. virtual addresses).
* **"What Every Programmer Should Know About Memory"** by Ulrich Drepper (Red Hat)
  * The definitive deep dive on RAM hardware, memory controllers, bus latency, and cache line prefetching.
* **"The Rust Programming Language" (The Book)**
  * [Chapter 4.3: Slices](https://doc.rust-lang.org/book/ch04-03-slices.html) – How Rust represents contiguous array views as fat pointers `(ptr, len)`.
  * [Chapter 8.1: Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) – Heap allocation, reallocation growth factors, and memory capacity.
* **V8 Engine Internals: "Elements kinds in V8"** by Mathias Bynens
  * [v8.dev/blog/elements-kinds](https://v8.dev/blog/elements-kinds)
  * An inside look at how modern JavaScript engines optimize contiguous arrays internally (Packed SMI vs. Double) and why introducing "holes" de-optimizes them into hash tables.

