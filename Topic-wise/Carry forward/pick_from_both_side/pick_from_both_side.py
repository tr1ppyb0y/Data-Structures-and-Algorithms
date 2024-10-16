"""
    Asked in Google.

    You are given an integer array.
    You have to perform k operations.
    In one operation, you can remove either the leftmost or the rightmost element of the array.
    Find and return the maximum possible sum of the k elements that were removed after the k operations.
"""

class DSA:
    def pick_from_both_side(self, arr, k):
        curr = res = sum(arr[-k:])
        for i in range(k):
            curr = curr - arr[-k+i] + arr[i]
            res = max(res, curr)
        return res


dsa = DSA()
arr, k = tuple(map(int, input().split())), int(input())
res = dsa.pick_from_both_side(arr, k)
print(f'{res} is the maximum sum.')
