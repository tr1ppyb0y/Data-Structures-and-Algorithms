"""
    You are given an integer array arr.
    You are also given a 2D integer array range with dimensions n x 2, where each row denotes a [l, r] query.
    For each query, you have to find the sum of all elements from l to r indices in arr (0 - indexed).
    More formally, find arr[l] + arr[l + 1] + arr[l + 2] +... + arr[r - 1] + arr[r] for each query.
"""

class DSA:

    def __init__(self, arr, interval) -> None:
        self.arr = arr
        self.interval = interval

    def in_place_prefix_sum(self):
        for idx, val in enumerate(self.arr[1:], start=1):
            self.arr[idx] = self.arr[idx-1] + val

    def range_sum_query(self):
        res = [0] * len(self.interval)

        # Construct the prefix sum array
        self.in_place_prefix_sum()

        # Calculate the range sum
        for idx, (l, r) in enumerate(self.interval):
            if l == 0:
                res[idx] = self.arr[r]
            else:
                res[idx] = self.arr[r] - self.arr[l - 1]
        return res


arr = list(map(int, input().split()))
interval = tuple(tuple(map(int, x.split(','))) for x in input().split())
dsa = DSA(arr, interval)
res = dsa.range_sum_query()
print(f'Range sum - {res} \nArray - {arr} \nRanges - {interval}')
