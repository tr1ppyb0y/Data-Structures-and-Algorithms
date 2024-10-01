from math import isqrt
class DSA:
    def count_factors(self, n):
        root = isqrt(n) + 1
        res = 1
        for i in range(2, root):
            res += (n % i == 0)
        return res * 2 - ((root - 1) * (root - 1) == n)

dsa = DSA()
n = int(input())
res = dsa.count_factors(n)
print(res)