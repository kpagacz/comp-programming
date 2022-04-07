import java.util.Comparator;
import java.util.PriorityQueue;

class Solution {
  public int lastStoneWeight(int[] stones) {
      PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Comparator.reverseOrder());
      for (int weight : stones)
        maxHeap.add(weight);
      while (maxHeap.size() > 1) {
        int first = maxHeap.remove();
        int second = maxHeap.remove();
        if (first != second) maxHeap.add(Math.abs(first - second));
      }
      if (maxHeap.isEmpty())
        return 0;
      else
        return maxHeap.peek();
  }
}
