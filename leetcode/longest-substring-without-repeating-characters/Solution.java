import java.util.HashMap;
import java.util.Map;

class Solution {
  public int lengthOfLongestSubstring(String s) {
    int answer = 0;
    Map<Character, Integer> positions = new HashMap<>();
    for (int left = 0, right = 0; right < s.length(); right++) {
      if (positions.containsKey(s.charAt(right))) {
        left = Math.max(left, positions.get(s.charAt(right)) + 1);
      }
      positions.put(s.charAt(right), right);
      answer = Math.max(answer, right - left + 1);
    }

    return answer;
  }
}
