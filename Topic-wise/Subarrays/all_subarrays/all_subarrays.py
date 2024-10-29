"""
    Given an array, print all sub-arrays.
"""

class DSA:
    def all_subarrays(self, arr):
        for i in range(len(arr)):
            for j in range(i, len(arr)):
                print(arr[i:j+1])



dsa = DSA()
arr = list(map(int, input().split()))
print(f"All sub-arrays of array {arr} are,")
dsa.all_subarrays(arr)
