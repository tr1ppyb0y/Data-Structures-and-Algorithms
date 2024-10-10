"""
    A wire connects N light bulbs.
    Each bulb has a switch associated with it; however, due to faulty wiring, 
    a switch also changes the state of all the bulbs to the right of the current bulb.
    Given an initial state of all bulbs, find the minimum number of switches 
    you have to press to turn on all the bulbs. You can press the same switch multiple times.
    Assume there will be atleast one bulb.
    Note: 0 represents the bulb is off and 1 represents the bulb is on.
"""

class DSA:
    def bulbs(self, arr):
        count = +(arr[0] == 0)
        curr = arr[0]
        for state in arr[1:]:
            if curr != state:
                count += 1
                curr = state
        return count

dsa = DSA()
arr = tuple(map(int, input().split()))
res = dsa.bulbs(arr)
print(f'{res} number of flips is required to onn all the bulbs with initial configuration {arr}')
