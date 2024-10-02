from math import isqrt
class DSA:
    def is_prime(self, n):
        root = isqrt(n) + 1
        for i in range(2, root):
            if n % i == 0:
                return False
        return True
        

dsa = DSA()
n = int(input())
res = dsa.is_prime(n)
print("Is prime number:", res)
