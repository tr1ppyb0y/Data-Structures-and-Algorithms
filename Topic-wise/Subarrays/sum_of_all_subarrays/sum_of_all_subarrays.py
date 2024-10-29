"""
    Given an array, find the sum of all sub-arrays.
"""

class DSA:
    def sum_of_all_subarrays(self, arr):
        n, summation = len(arr), 0
        for idx, val in enumerate(arr):
            summation += (idx + 1) * (n - idx) * val
        return summation

dsa = DSA()
arr = list(map(int, input().split()))
summation = dsa.sum_of_all_subarrays(arr)
print(f"Sum of all subarrays: {summation}")
