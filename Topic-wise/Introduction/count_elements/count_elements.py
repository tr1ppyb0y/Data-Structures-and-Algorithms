'''Given an array integers. Count the number of elements that have at least 1 elements greater than itself.'''

class DSA:
    def count_elements(self, arr):
        maximum = max(arr)
        count = 0
        for num in arr:
            count += num < maximum
        return count

dsa = DSA()
arr = tuple(map(int, input().split()))
res = dsa.count_elements(arr)
print(res)
