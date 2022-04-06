/**
 * Definition for singly-linked list.
 * public class ListNode {
 * int val;
 * ListNode next;
 * ListNode() {}
 * ListNode(int val) { this.val = val; }
 * ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
  public ListNode swapNodes(ListNode head, int k) {
    int total = 0;
    ListNode it = head;
    while (it != null) {
      total++;
      it = it.next;
    }

    int first = k, second = total - k + 1;
    if (first > second) {
      int temp = first;
      first = second;
      second = temp;
    }
    it = head;
    ListNode firstNode = head;
    int counter = 1;
    while (counter < first) {
      counter++;
      firstNode = firstNode.next;
    }
    ListNode secondNode = firstNode;
    while (counter < second) {
      counter++;
      secondNode = secondNode.next;
    }
    int temp = firstNode.val;
    firstNode.val = secondNode.val;
    secondNode.val = temp;

    return head;
  }
}
