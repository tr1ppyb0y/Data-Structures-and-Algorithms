'''
    You have given a string having Uppercase English letters.
    You have to find the number of pairs (i, j) 
    such that A[i] = 'A', A[j] = 'G' and i < j.
'''

class DSA:
    def special_subsequences(self, string):
        count = res = 0
        for char in string:
            if char == 'G':
                res += count
            else:
                count += 1
        return res

dsa = DSA()
string = input()
res = dsa.special_subsequences(string)
print(f"String {string} contains {res} AG pairs")
