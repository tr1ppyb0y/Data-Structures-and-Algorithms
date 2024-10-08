class DSA:
    def perfect_numbers(self, n):
        res = []
        for i in range(1, n):
            if n % i == 0:
                res.append(i)
        return sum(res) == n

dsa = DSA()
n = int(input())
res = dsa.perfect_numbers(n)
print(f'Is {n} perfect number?', res)
