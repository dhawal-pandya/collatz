package main

import (
	"fmt"
	"math/big"
	"sync"
)

// cache to store previously computed Collatz sequences
var collatzCache = make(map[string][]*big.Int)
var cacheLock sync.Mutex // handle concurrent access to the cache

// collatzSequence calculates the sequence for a given number using caching
func collatzSequence(n *big.Int) []*big.Int {
	original := new(big.Int).Set(n) // preserve the original value
	sequence := []*big.Int{}

	for n.Cmp(big.NewInt(1)) != 0 {
		sequence = append(sequence, new(big.Int).Set(n))

		// check if this number's sequence is cached
		cacheLock.Lock()
		if cachedSeq, found := collatzCache[n.String()]; found {
			cacheLock.Unlock()
			sequence = append(sequence, cachedSeq...)
			goto CacheSave
		}
		cacheLock.Unlock()

		//
		if new(big.Int).Mod(n, big.NewInt(2)).Cmp(big.NewInt(0)) == 0 { // when n is even
			n.Div(n, big.NewInt(2)) // n = n / 2
		} else { // when n is odd
			n.Mul(n, big.NewInt(3)).Add(n, big.NewInt(1)) // n = 3 * n + 1
		}
	}

	sequence = append(sequence, big.NewInt(1)) // append the last element of the sequence

CacheSave:
	cacheLock.Lock()
	collatzCache[original.String()] = append([]*big.Int{}, sequence[1:]...)
	cacheLock.Unlock()

	return sequence
}

func main() {
	var input string
	fmt.Print("Enter a number to calculate Collatz sequences up to: ")
	fmt.Scan(&input)

	num := new(big.Int)
	if _, ok := num.SetString(input, 10); !ok {
		fmt.Println("Invalid number. Please enter a valid integer.")
		return
	}

	for i := big.NewInt(1); i.Cmp(num) <= 0; i.Add(i, big.NewInt(1)) {
		sequence := collatzSequence(new(big.Int).Set(i))
		fmt.Printf("Collatz sequence for %s: ", i.String())
		for _, step := range sequence {
			fmt.Print(step.String(), " ")
		}
		fmt.Println()
	}
}
