#include <iostream>
#include <queue>
#include <vector>
using namespace std;

struct TimeInt {
    int start;
    int end;
    TimeInt(int s, int e) {
        start = s;
        end = e;
    }
};

bool operator> (const TimeInt &c1, const TimeInt &c2) {
    return c1.start > c2.start;
}

bool operator>= (const TimeInt &c1, const TimeInt &c2) {
    return c1.start >= c2.start;
}

bool operator< (const TimeInt &c1, const TimeInt &c2) {
    return c1.start < c2.start;
}

bool operator<= (const TimeInt &c1, const TimeInt &c2) {
    return c1.start <= c2.start;
}

int main(int argc, char *argv[])
{
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int t;
    cin >> t;
    for (int ti = 0; ti < t; ++ti) {
        int n, bot_runtime_max;
        cin >> n;
        cin >> bot_runtime_max;

        priority_queue<TimeInt, vector<TimeInt>, greater<TimeInt>> tis;
        int t_end_max = 1;
        for (int i = 0; i < n; ++i) {
            int t_start, t_end;
            cin >> t_start;
            cin >> t_end;

            auto ti = TimeInt(t_start, t_end);
            tis.push(ti);

            t_end_max = max(t_end, t_end_max);
        }

        bool bot_active = false;
        int bot_start = 1;
        int n_bots = 0;
        TimeInt next_ti = tis.top();
        tis.pop();
        for (int i = 1; i <= t_end_max; ++i) {
            if (bot_active) {
                int bot_runtime = i - bot_start;
                if (bot_runtime >= bot_runtime_max) {
                    bot_active = false;
                }
            }

            if (i >= next_ti.start && i < next_ti.end) {
                if (!bot_active) {
                    bot_active = true;
                    bot_start = i;
                    n_bots++;
                }
            } else if (i >= next_ti.end) {
                next_ti = tis.top();
                tis.pop();
            }
        }

        cout << "Case #" << ti + 1 << ": " << n_bots << endl;
    }

    return 0;
}

void heapProto()
{
    priority_queue<TimeInt, vector<TimeInt>, greater<TimeInt>> q;
    q.push(TimeInt(4, 5));
    q.push(TimeInt(8, 11));
    q.push(TimeInt(13, 15));
    q.push(TimeInt(17, 18));
    while (!q.empty()) {
        cout << q.top().start << " ";
        q.pop();
    }
    cout << endl;
}
