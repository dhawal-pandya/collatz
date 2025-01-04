## Collatz Conjecture
- This repository contains implementations of the Collatz Conjecture in both Rust and Go. The Collatz Conjecture, also known as the 3x+1 problem, is a mathematical conjecture that remains unsolved. It has perplexed mathematicians for years due to its simple formulation and seemingly unpredictable behavior.

### Warning
- While the Collatz Conjecture is a fascinating subject, attempting to prove or disprove it is no small feat. Many current-day mathematicians consider it a very challenging problem that may never be fully resolved. Proceed with caution—mathematical endeavors like this are not for the faint of heart.
- Just go away actually...


### Collatz Conjecture Overview
The Collatz Conjecture states that for any positive integer, the following steps will eventually lead to the number 1:

1. If the number is even, divide it by 2.
2. If the number is odd, multiply it by 3 and add 1.
3. Repeat the process for the resulting number.
- It’s believed by many mathematicians that no matter how large the starting number is, the sequence will always eventually reach 1, though a formal proof remains elusive.

You can read more about the [Collatz Conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture)

### Code Implementation
- Languages: Rust and Go
- Functionality: Both implementations calculate and print the Collatz sequence for a given number. Caching is used to optimize performance by storing previously calculated sequences.
