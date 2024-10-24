"""
    Asked in Adobe, Amazon, Hike.

    You are given an array of integers.
    Your task is to find the equilibrium index of the given array
    The equilibrium index of an array is an index such that,
    the sum of elements at lower indexes is equal to the sum of elements at higher indexes.
    If there are no elements at lower or higher,
    then the corresponding sum of elements is considered as 0.
"""

from itertools import accumulate

class DSA:
    def equilibrium_index_of_an_array(self, arr):
        prefix = tuple(accumulate(arr))
        suffix = tuple(accumulate(arr[::-1]))[::-1]

        curr = 0
        for idx, val in enumerate(prefix[:-1]):
            if curr == suffix[idx + 1]:
                return idx
            curr = val
        
        return -1 if curr != 0 else len(arr) - 1



dsa = DSA()
arr = tuple(map(int, input().split()))
res = dsa.equilibrium_index_of_an_array(arr)
if res == -1:
    print(f'There is no Equilibrium index in array {arr}.')
else:
    print(f'Equilibrium index of array {arr} is, {res} [0-indexed].')
