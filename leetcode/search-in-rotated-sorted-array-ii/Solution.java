import java.util.Arrays;

class Solution {
  private boolean binarySearch(int[] nums, int left, int right, int target) {
    while (left <= right) {
      int mid = (right - left) / 2 + left;
      if (nums[mid] == target)
        return true;
      if (nums[mid] < target) {
        left = mid + 1;
      } else {
        right = mid;
      }
    }

    return false;
  }
  public boolean search(int[] nums, int target) {
    nums = Arrays.stream(nums).distinct().toArray();
    int left = 0, right = nums.length - 1;
    int pivot = -1;
    while (left < right) {
      int mid = (right - left) / 2 + left;
      if (mid == nums.length - 1) {
        pivot = mid;
        break;
      }
      if (nums[mid] > nums[mid + 1]) {
        pivot = mid;
        break;
      } else {
        if (nums[left] > nums[mid]) {
          right = mid;
        } else {
          left = mid + 1;
        }
      }
    }

    System.out.println(pivot);
    return binarySearch(nums, 0, pivot, target) || binarySearch(nums, pivot + 1, nums.length - 1, target);
  }
}
