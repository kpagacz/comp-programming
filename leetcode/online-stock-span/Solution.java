// link to the problem: https://leetcode.com/problems/online-stock-span
import java.util.ArrayList;
import java.util.Stack;

class StockSpanner {
  Stack<Integer> monotonicStack;
  ArrayList<Integer> prices;
  public StockSpanner() {
    monotonicStack = new Stack<>();
    prices = new ArrayList<>();
  }

  public int next(int price) {
    prices.add(price);
    while (!monotonicStack.empty() && prices.get(monotonicStack.peek()) <= price) {
      monotonicStack.pop();
    }
    int answer;
    if (monotonicStack.empty()) {
      answer = prices.size();
    } else {
      answer = prices.size() - monotonicStack.peek() - 1;
    }
    monotonicStack.push(prices.size() - 1);
    return answer;
  }
}

/**
* Your StockSpanner object will be instantiated and called as such:
* StockSpanner obj = new StockSpanner();
* int param_1 = obj.next(price);
*/
