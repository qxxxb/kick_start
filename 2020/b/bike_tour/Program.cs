using System;
using System.Collections.Generic;

namespace bike_tour
{
    class Program
    {
        static void Main(string[] args)
        {
            string n_str = Console.ReadLine();
            int t = int.Parse(n_str);
            for (int i = 0; i < t; i++)
            {
                int n = int.Parse(Console.ReadLine());
                string hs_str = Console.ReadLine();
                string[] hss_str = hs_str.Split(null);

                List<int> hs = new List<int>();
                for (int j = 0; j < hss_str.Length; j++)
                {
                    int h = int.Parse(hss_str[j]);
                    hs.Add(h);
                }

                int n_peaks = 0;
                for (int j = 1; j < hs.Count - 1; j++)
                {
                    int h = hs[j];
                    if (h > hs[j - 1] && h > hs[j + 1]) {
                        n_peaks++;
                    }
                }

                Console.WriteLine("Case #{0}: {1}", i, n_peaks);
            }
        }
    }
}
