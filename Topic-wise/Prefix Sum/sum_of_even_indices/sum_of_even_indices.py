"""
    You are given an array and queries. 
    Each query consists of two integers left and right.
    For every query, the task is to calculate the sum of all even indices from left to right.
"""

class DSA:
    def sum_of_even_indices(self, arr, queries):
        even = [arr[0]] * len(arr)
        for idx, val in enumerate(arr[1:], start=1):
            even[idx] = even[idx - 1] + val * (idx % 2 == 0)

        res = [0] * len(queries)
        for idx, (l, r) in enumerate(queries):
            if l == 0:
                res[idx] = even[r]
            else:
                res[idx] = even[r] - even[l - 1]
        return res



dsa = DSA()
arr = tuple(map(int, input().split()))
queries = tuple(tuple(map(int, x.split(','))) for x in input().split())
res = dsa.sum_of_even_indices(arr, queries)
print("Sum of numbers at even indices -")
print(f"Array: {arr}\nQueries: {queries}\nResult: {res}")

