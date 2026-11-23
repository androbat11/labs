## Arrays in memory

In physical memory (RAM), an array is a contiguous sequence of bytes.

* Memory Address: 0x1000 0x1004 0x1008 0x100C
* Element Index: [0]    [1]     [2]    [3]
* Value (i32):   | 10 | | 25 |  | 42 |  | 99 |

Address computation:

Address(i) = Base Address + (i x S)

* Rust:
  * A dynamic array Vec<T> is a 24-byte stack structure (on 64 bit systems) containing three words (ptr, capacity, length) pointing to the contiguous heap buffer.
  * Iterating linearly maximizes CPU L1/L2 cache lines (loading 64 bytes at once, triggering hardware prefetching).



## Questions:
* What is the explanation of Address(i) = Base Address + (i X S)?
 
