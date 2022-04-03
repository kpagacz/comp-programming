import java.util.Stack;

class Solution {
  public int maxWidthRamp(int[] nums) {
    int answer = 0;
    Stack<Integer> monotonicStack = new Stack<>();
    for (int i = 0; i < nums.length; i++)
      if (monotonicStack.empty() || nums[i] < nums[monotonicStack.peek()])
        monotonicStack.push(i);

    for (int i = nums.length - 1; i > answer; i--)
      while (!monotonicStack.empty() && nums[i] > monotonicStack.peek())
        answer = Math.max(answer, i - monotonicStack.pop());

    return answer;
  }
}
