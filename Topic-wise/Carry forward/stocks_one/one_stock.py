"""
    Say you have an array, for which the ith element is the price of a given stock on day i.
    If you were only permitted to complete at most one transaction (ie, buy one and sell one share of the stock), 
    design an algorithm to find the maximum profit. Assume trading happend atleast for one day.
    Return the maximum possible profit.
"""

class DSA:
    def one_stock(self, arr):
        buy = sell = arr[0]
        profit = 0
        for price in arr[1:]:
            buy = min(buy, price)
            profit = max(profit, price - buy)
        return profit

dsa = DSA()
arr = tuple(map(int, input().split()))
res = dsa.one_stock(arr)
print(f'{res} is the maximum profit that can be made.')
