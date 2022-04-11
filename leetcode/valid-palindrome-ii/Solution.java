// link to the problem: https://leetcode.com/problems/valid-palindrome-ii/submissions/

// What have I learned?
// I had a bug where I was comparing strings via == instead of String.equals. Turns out == compares references instead
// of values...

class Solution {
  private boolean isPalindrome(String s) {
    return s.equals(new StringBuilder(s).reverse().toString());
  }
  public boolean validPalindrome(String s) {
    int left = 0, right = s.length() - 1;
    while (left < right) {
      if (s.charAt(left) != s.charAt(right)) {
        return isPalindrome(s.substring(left + 1, right + 1)) || isPalindrome(s.substring(left, right));
      }
      right--;
      left++;
    }

    return true;
  }
}
