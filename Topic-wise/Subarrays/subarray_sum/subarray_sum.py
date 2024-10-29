"""
    Given an array, print sum of all sub-arrays.
"""

class DSA:
    def subarray_sum(self, arr):
        for i in range(len(arr)):
            for j in range(i, len(arr)):
                print(f"{arr[i:j+1]} - {sum(arr[i:j+1])}")



dsa = DSA()
arr = list(map(int, input().split()))
print(f"Sub-arrays sum of array {arr} are,")
dsa.subarray_sum(arr)
