"""
    Given an array arr, find the maximum subarray sum out of all possible subarray of length k.
"""

class DSA:
    def maximum_subarray_sum(self, arr, k):
        curr = res = sum(arr[:k])
        n = len(arr)

        for i in range(k, n):
            curr += arr[i] - arr[i - k]
            res = max(res, curr)
        
        return res
        

dsa = DSA()
arr, k = list(map(int, input().split())), int(input())
res = dsa.maximum_subarray_sum(arr, k)
print(f"Maximum subarray sum out of all possible subarray of length k is: {res}")
