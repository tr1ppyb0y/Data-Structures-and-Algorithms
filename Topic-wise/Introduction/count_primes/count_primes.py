from math import isqrt
class DSA:
    def count_primes(self, n):
        primes = [True] * (n+1)
        primes[0] = primes[1] = False
        for idx, val in enumerate(primes[2:], start=2):
            if val:
                for i in range(idx * idx, n+1, idx):
                    primes[i] = False
        return sum(primes)
            

dsa = DSA()
n = int(input())
res = dsa.count_primes(n)
print(f'There are {res} primes less than or equal to {n}')
