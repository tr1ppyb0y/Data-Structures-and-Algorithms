"""
    Given an array, find the sub-array with maximum sum.
"""

class DSA:
    def maximum_sum_subarray(self, arr):
        x = y = 0
        maximum, n = arr[0], len(arr)
        for i in range(n):
            summation = arr[i]
            if summation > maximum:
                maximum = summation
                x = y = i
            for j in range(i + 1, n):
                summation += arr[j]
                if summation > maximum:
                    maximum = summation
                    x, y = i, j
        return x, y, maximum


dsa = DSA()
arr = list(map(int, input().split()))
l, r, maximum = dsa.maximum_sum_subarray(arr)
print(f"Sub-array {arr[l: r + 1]} with maximum sum of {maximum} for array, {arr}.")
