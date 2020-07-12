using System;
using System.Collections.Generic;

namespace bus_routes
{
    class Program
    {
        static void Main(string[] args)
        {
            int t = int.Parse(Console.ReadLine());
            for (int k = 0; k < t; k++)
            {
                string nd_str = Console.ReadLine();
                string[] nd_strs = nd_str.Split(null);
                int n = int.Parse(nd_strs[0]);
                int d = int.Parse(nd_strs[1]);

                string xs_str = Console.ReadLine();
                string[] xs_strs = xs_str.Split(null);

                List<int> xs = new List<int>(xs_strs.Length);
                for (int j = 0; j < xs_strs.Length; j++)
                {
                    int h = int.Parse(xs_strs[j]);
                    xs.Add(h);
                }

                int di = d;
                for (int i = xs.Count - 1; i >= 0; i--)
                {
                    int x = xs[i];
                    di -= di % x;
                }

                Console.WriteLine("Case #{0}: {1}", k + 1, di);
            }
        }
    }
}
