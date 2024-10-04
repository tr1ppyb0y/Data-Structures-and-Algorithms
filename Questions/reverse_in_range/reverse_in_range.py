class DSA:
    def reverse_in_range(self, arr, a, b):
        return arr[:a] + arr[a:b+1][::-1] + arr[b+1:]
        

dsa = DSA()
arr, a, b = list(map(int, input().split())), list(map(int, input().split()))
res = dsa.reverse_in_range(arr, a, b)
print(f'Reversed {arr} in between {a}, {b} [0-indexed]', res)
