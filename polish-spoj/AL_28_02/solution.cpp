// link to the problem https://pl.spoj.com/problems/AL_28_02/
#include<iostream>
#include<vector>
#include<algorithm>

struct QueueTime {
	long long exec_time, queue_time;
	QueueTime(long long _exec_time, long long _queue_time) : exec_time(_exec_time), queue_time(_queue_time) {}
	QueueTime() = default;
	QueueTime(const QueueTime& other) : exec_time(other.exec_time), queue_time(other.queue_time) {}
};

void test_case(int no) {
	auto comparator = [](QueueTime first, QueueTime second) {return first.queue_time > second.queue_time; };
	long long A, B, C, a, b, c;
	long long answer = 0;
	std::cin >> A >> B >> C >> a >> b >> c;
	std::vector<QueueTime> queue_times = { QueueTime(a, A * a), QueueTime(b, b * B), QueueTime(c, c * C) };
	std::make_heap(queue_times.begin(), queue_times.end(), comparator);
	while (queue_times.size() > 0) {
		QueueTime smallest_queue = queue_times[0];
		answer += smallest_queue.exec_time + smallest_queue.queue_time;
		std::transform(
			queue_times.begin(),
			queue_times.end(),
			queue_times.begin(),
			[&](QueueTime queue_time) {queue_time.queue_time = std::max((long long)(0), queue_time.queue_time - smallest_queue.queue_time - smallest_queue.exec_time); return queue_time; }
		);
		std::pop_heap(queue_times.begin(), queue_times.end(), comparator);
		queue_times.pop_back();
	}

	std::cout << answer << "\n";
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case(tests);
		--tests;
	}
}