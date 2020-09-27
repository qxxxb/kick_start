#include <iostream>
#include <queue>
using namespace std;

struct Person {
    int id;
    int amt;
};

int main(int argc, char *argv[])
{
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int t;
    cin >> t;
    for (int ti = 0; ti < t; ++ti) {
        int n, max_amt;
        cin >> n >> max_amt;

        queue<Person> line;
        for (int i = 0; i < n; ++i) {
            int a_i;
            cin >> a_i;
            Person p = { .id = i + 1, .amt = a_i };
            line.push(p);
        }

        queue<int> leave_order;

        while (line.size() > 0) {
            auto p = line.front();
            line.pop();

            if (p.amt <= max_amt) {
                p.amt = 0;
                leave_order.push(p.id);
            } else {
                p.amt -= max_amt;
                line.push(p);
            }
        }

        cout << "Case #" << ti + 1 << ": ";
        while (leave_order.size() > 0) {
            int id = leave_order.front();
            leave_order.pop();
            cout << id << " ";
        }
        cout << endl;
    }
    return 0;
}
