import java.util.HashMap;

class Solution {
  public boolean makeEqual(String[] words) {
    HashMap<Character, Integer> characterCounts = new HashMap<>();
    for (String word : words)
      for (char c : word.toCharArray())
        if (characterCounts.putIfAbsent(c, 1) != null)
          characterCounts.merge(c, 1, Integer::sum);

    return characterCounts.values().stream().map(value -> value.intValue() % words.length == 0).reduce(true,
        (value, element) -> value && element);
  }
}
