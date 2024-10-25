"""
    You are given an array and queries with left and right indices.
    For every query, find the count of even numbers for every query in that range.
"""

from itertools import accumulate

class DSA:
    def even_numbers_in_range(self, arr, queries):
        acc = tuple(accumulate(+(x % 2 == 0) for x in arr))
        res = [0] * len(queries)

        for idx, (l, r) in enumerate(queries):
            if l == 0:
                res[idx] = acc[r]
            else:
                res[idx] = acc[r] - acc[l - 1]
        return res



dsa = DSA()
arr = tuple(map(int, input().split()))
queries = tuple(tuple(map(int, x.split(','))) for x in input().split())
res = dsa.even_numbers_in_range(arr, queries)
print(f"Array: {arr}\nQueries: {queries}\nResult: {res}")
