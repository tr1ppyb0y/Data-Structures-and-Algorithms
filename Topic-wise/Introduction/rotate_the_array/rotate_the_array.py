class DSA:
    def reverse_the_array(self, arr, b):
        b = b % len(arr)
        return arr[-b:] + arr[:-b] 
        

dsa = DSA()
arr, b = list(map(int, input().split())), int(input())
res = dsa.reverse_the_array(arr, b)
print(f'Rotated array ', res)
