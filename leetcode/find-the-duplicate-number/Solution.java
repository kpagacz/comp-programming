import java.util.Arrays;

class Solution {
    public int findDuplicate(int[] nums) {
        return Arrays.stream(nums).sum() - (nums.length - 1) * nums.length / 2;
    }
}
