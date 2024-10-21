"""
    Given an array A. Construct prefix sum of the array in the given array itself.
"""

class DSA:
    def in_place_prefix_sum(self, arr):
        for idx, val in enumerate(arr[1:], start=1):
            arr[idx] = arr[idx-1] + val
        return arr

dsa = DSA()
arr = list(map(int, input().split()))
orignal = tuple(arr)
dsa.in_place_prefix_sum(arr)
print(f'{arr} is the prefix of array, {orignal}')
