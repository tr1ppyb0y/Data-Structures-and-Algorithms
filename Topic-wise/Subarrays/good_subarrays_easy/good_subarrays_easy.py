"""
    Amazon, Meta.

    Given an array of integers arr and a value k, a subarray of an array is said to be good if it fulfills any one of the criteria:
        1. Length of the subarray is be even, and the sum of all the elements of the subarray must be less than k.
        2. Length of the subarray is be odd, and the sum of all the elements of the subarray must be greater than k.
        Your task is to find the count of good subarrays in arr.
    
    Note: 1 <= len(arr) <= 10^3
"""

class DSA:
    def good_subarrays_easy(self, arr, k):
        n, count = len(arr), 0
        for i in range(n):
            summation = 0
            for j in range(i, n):
                summation += arr[j]
                count += (summation < k) * ((j - i + 1) % 2 == 0)
                count += (summation > k) * ((j - i + 1) % 2 == 1)
        return count

dsa = DSA()
arr, k = list(map(int, input().split())), int(input())
res = dsa.good_subarrays_easy(arr, k)
print(f"Number of good sub-arrays are: {res}")


"""
P.S.: If either less than or greater than k is asked,
then an if condition with break can be used to break early.
Given, arr[i] >= 0.
"""
