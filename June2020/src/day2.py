# Annoyingly, no Rust of this, probably because ownership is super janky
# Will do my submission in Py3

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def deleteNode(self, node):
        """
        :type node: ListNode
        :rtype: void Do not return anything, modify node in-place instead.
        """
        node.val = node.next.val
        node.next = node.next.next

def print_list(node):
    while node is not None:
        print("{} ".format(node.val))
        node = node.next

if __name__ == "__main__":
    n4 = ListNode(9)
    n3 = ListNode(1)
    n2 = ListNode(5)
    n1 = ListNode(4)
    n1.next = n2
    n2.next = n3
    n3.next = n4

    print("List: ")
    print_list(n1)

    sol = Solution()
    sol.deleteNode(n3)
    print("List after deleting node 3: ")
    print_list(n1)