"""
    You are given an array and queries. 
    Each query consists of two integers left and right.
    For every query, the task is to calculate the sum of all odd indices from left to right.
"""

class DSA:
    def sum_of_odd_indices(self, arr, queries):
        odd = [0] * len(arr)
        for idx, val in enumerate(arr[1:], start=1):
            odd[idx] = odd[idx - 1] + val * (idx % 2)

        res = [0] * len(queries)
        for idx, (l, r) in enumerate(queries):
            if l == 0:
                res[idx] = odd[r]
            else:
                res[idx] = odd[r] - odd[l - 1]
        return res



dsa = DSA()
arr = tuple(map(int, input().split()))
queries = tuple(tuple(map(int, x.split(','))) for x in input().split())
res = dsa.sum_of_odd_indices(arr, queries)
print("Sum of numbers at odd indices -")
print(f"Array: {arr}\nQueries: {queries}\nResult: {res}")

