
# The task https://leetcode.com/problems/linked-list-cycle/

# Given head, the head of a linked list, determine if the linked list has a cycle in it.
# There is a cycle in a linked list if there is some node in the list that can be reached again by
# continuously following the next pointer. Internally, pos is used to denote the index of
# the node that tail's next pointer is connected to


# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#        self.val = x
#        self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:

        item = head
        result = {}

        if head is None:
            return False

        while item.next is not None:
            if item in result:
                result[item].append(item.val)
                return True
            else:
                result[item] = [item.val]
            item = item.next
        return False






