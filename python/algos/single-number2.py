
# The task https://leetcode.com/problems/single-number-ii/

# Given an integer array nums where every element appears three times except for one,
# which appears exactly once. Find the single element and return it.


# Input: nums = [2,2,3,2]
# Output: 3

class Solution:
    def singleNumber(self, nums: List[int]) -> int:
      return (3 * sum(set(nums)) - sum(nums)) // 2