'''
    Given an integer array containing n distinct integers, 
    you have to find all the leaders in array. 
    An element is a leader if it is strictly greater than all the elements to its right side.
    NOTE: The rightmost element is always a leader.
'''

class DSA:
    def leader_in_array(self, arr):
        leaders = [arr[-1]]
        for val in arr[::-1][1:]:
            if val > leaders[-1]:
                leaders.append(val)
        return leaders

dsa = DSA()
arr = tuple(map(int, input().split()))
res = dsa.leader_in_array(arr)
print(f'{res} are the leaders in array {arr}')
