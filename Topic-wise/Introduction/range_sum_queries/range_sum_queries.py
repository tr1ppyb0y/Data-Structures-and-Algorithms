from itertools import accumulate
class DSA:
    def range_sum_queries(self, m, n, arr, queries):
        acc = tuple(accumulate(arr))
        res = [0] * n
        for idx, query in enumerate(queries):
            if query[0] == 0:
                res[idx] += acc[query[1]]
            else:
                res[idx] += (acc[query[1]] - acc[query[0]-1])
        return res

dsa = DSA()
m, n = map(int, input().split())
arr = list(map(int, input().split()))
queries = [tuple(map(int, input().split())) for _ in range(n)]
res = dsa.range_sum_queries(m, n, arr, queries)
print(arr)
for i in range(n):
    print(queries[i], res[i])
