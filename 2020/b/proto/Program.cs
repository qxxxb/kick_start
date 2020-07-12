using System;

namespace proto
{
    class Program
    {
        // static void Main(string[] args)
        // {
        //     Console.Write("Enter something idiot: ");
        //     string s = Console.ReadLine();
        //     int i;
        //     if (int.TryParse(s, out i)) {
        //         Console.WriteLine("i: {0}", i);
        //     } else {
        //         Console.WriteLine("s: {0}", s);
        //     }
        // }

        static void Main(string[] args)
        {
            foreach (var arg in args)
            {
                Console.WriteLine("arg: {0}", arg);
            }
        }
    }
}
