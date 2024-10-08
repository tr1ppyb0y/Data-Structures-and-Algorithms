class DSA:
    def reverse_the_array(self, arr):
        n, arr = len(arr), list(arr)
        for i in range(0, n//2):
            arr[i], arr[n-i-1] = arr[n-i-1], arr[i]
        return arr       
        

dsa = DSA()
arr = list(map(int, input().split()))
res = dsa.reverse_the_array(arr)
print(f'Reversed array ', res)
