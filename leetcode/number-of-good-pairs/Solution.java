import java.util.Arrays;

class Solution {
  public int numIdenticalPairs(int[] nums) {
    int[] counts = new int[101];
    Arrays.fill(counts, 0);
    for (int num : nums)
      counts[num]++;
    int answer = 0;
    for (int count : counts)
      answer += count * (count - 1) / 2;
    return answer;
  }
}
