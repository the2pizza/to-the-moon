
# The task https://leetcode.com/problems/single-number/
# Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

# Input: nums = [2,2,1]
# Output: 1

class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        result = 0
        for x in nums:
            result = x ^ result
        return result