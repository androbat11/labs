# Topic 01: Arrays & Strings

Welcome to the foundational module of the Algorithms & Data Structures Lab.

## 📖 Theoretical Foundations & Study Notes
1. [00_recommended_readings_guide.md](file:///home/manuel/Documents/labs/algorith_data_structures/topics/01_arrays_and_strings/00_recommended_readings_guide.md)
   - Comprehensive reading syllabus: exact textbooks, chapter numbers, section breakdowns, and topics.
   - CS:APP (address arithmetic, scaled indexing, 64-byte cache lines).
   - OSTEP (virtual memory, base & bounds address translation, memory APIs).
   - Ulrich Drepper (RAM bus controllers, L1/L2/L3 caches, hardware prefetchers).
   - Rust systems memory (`Vec<T>` heap triplets, `&[T]` fat pointers, UTF-8 variable width).
   - V8 engine internals (Packed SMI/Double arrays vs. Dictionary mode).

2. [01_memory_layout_and_indexing.md](file:///home/manuel/Documents/labs/algorith_data_structures/topics/01_arrays_and_strings/01_memory_layout_and_indexing.md)
   - Physical RAM as byte-addressable space.
   - Address calculation formula $\text{Base} + (i \times S)$.
   - The math logic: multiplication as repeated addition & dimensional unit cancellation.
   - From address to value (dereferencing, assembly `mov eax, [rbx + rdi*4]`, locker analogy).
   - 0-indexing as an offset metric.
   - Maximum bounds and memory footprint.
   - CPU Cache Lines, Spatial Locality, and $O(1)$ indexing.
   - Rust (`[T; N]`, `Vec<T>`) vs. Bun/TypeScript (V8/JSC packed arrays vs. dictionary mode).

3. [02_indexing_math_workbook.md](file:///home/manuel/Documents/labs/algorith_data_structures/topics/01_arrays_and_strings/02_indexing_math_workbook.md)
   - Interactive exercises & problem workbook covering:
     - Part 1: Conceptual reasoning & dimensional unit checks.
     - Part 2: Manual address calculation drills (decimal & hexadecimal).
     - Part 3: Boundaries, memory footprints, and sentinels.
     - Part 4: "Bug Detective" diagnostics (stride errors, addition bugs).
     - Part 5: Hardware cache line calculations & hit/miss ratios.

---

## 📚 Recommended Reading Materials

To deepen your systems-level intuition regarding memory and arrays, these are the highest-quality references:

### 1. The Gold Standard Textbook
* **"Computer Systems: A Programmer's Perspective" (CS:APP)** by Randal E. Bryant & David R. O'Hallaron
  * *Must-read sections*:
    * **Section 3.8: Array Allocation and Access** (breaks down 1D/2D array address arithmetic at machine/assembly level).
    * **Chapter 6: The Memory Hierarchy** (explains CPU caches, cache lines, and locality).

### 2. Free High-Quality Online Books & Papers
* **"Operating Systems: Three Easy Pieces" (OSTEP)** by Remzi Arpaci-Dusseau and Andrea Arpaci-Dusseau
  * Free online: [ostep.org](https://pages.cs.wisc.edu/~remzi/OSTEP/)
  * *Must-read chapters*: Virtual Memory, Memory Virtualization, and Address Translation.
* **"What Every Programmer Should Know About Memory"** by Ulrich Drepper (Red Hat)
  * The definitive whitepaper explaining RAM architectures, memory controllers, L1/L2/L3 caches, and why sequential array access dominates random node access.
* **"The Rust Programming Language" (The Book)**
  * [Chapter 4: Understanding Ownership (Slices)](https://doc.rust-lang.org/book/ch04-03-slices.html)
  * [Chapter 8: Common Collections (Vectors)](https://doc.rust-lang.org/book/ch08-01-vectors.html)
* **V8 Engine Internals: "Elements kinds in V8"** by Mathias Bynens
  * [v8.dev/blog/elements-kinds](https://v8.dev/blog/elements-kinds)
  * How JavaScript arrays work behind the scenes (Packed SMI vs Double vs Dictionary).

---

## 🛠️ Problems & Implementation Lab

| Problem | Pattern | Difficulty | Rust | TypeScript |
|---|---|:---:|:---:|:---:|
| **Two Sum II (Sorted Array)** | Two Pointers (Opposite Direction) | Easy / Med | [rust/src/two_sum_ii.rs](file:///home/manuel/Documents/labs/algorith_data_structures/topics/01_arrays_and_strings/rust) | [ts/two_sum_ii.test.ts](file:///home/manuel/Documents/labs/algorith_data_structures/topics/01_arrays_and_strings/ts) |
