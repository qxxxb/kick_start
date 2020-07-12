using System;

namespace robot_path
{
    class Program
    {
        const int max = 1000000000;

        static void Shit(char c, out int dx, out int dy)
        {

            dx = 0;
            dy = 0;

            switch (c) {
                case 'S':
                    dy = 1;
                    break;
                case 'N':
                    dy = -1;
                    break;
                case 'E':
                    dx = 1;
                    break;
                case 'W':
                    dx = -1;
                    break;
            }

            return;
        }

        static int Mod(int a, int b)
        {
            if (a > 0) {
                return a % b;
            } else if (a < 0) {
                return b - ((-a) % b);
            } else {
                // a == 0
                return 0;
            }
        }

        static int x = 1;
        static int y = 1;

        static void Exp(
            string prog, int p,
            out int dx, out int dy, out int p_exp
        ) {
            dx = 0;
            dy = 0;
            int n = 1;
            int i = p;
            while (i < prog.Length)
            {
                char c = prog[i];
                switch (c)
                {
                    case '(':
                        int sub_p;
                        int ddx;
                        int ddy;
                        Exp(prog, i + 1, out ddx, out ddy, out sub_p);
                        dx += (ddx * n);
                        dy += (ddy * n);
                        // dx = Mod(ddx * n, (int)max - 1) + 1;
                        // dy = Mod(ddy * n, (int)max - 1) + 1;
                        i = sub_p + 1;
                        break;
                    case ')':
                        p_exp = i;
                        return;
                    default:
                        if (c >= '2' && c <= '9') {
                            n = c - '0';
                        } else {
                            int ddx_a;
                            int ddy_a;
                            Shit(c, out ddx_a, out ddy_a);
                            dx += ddx_a;
                            dy += ddy_a;
                        }
                        i++;
                        break;
                }
            }

            p_exp = i;
            return;
        }

        static void Main(string[] args)
        {
            int t = int.Parse(Console.ReadLine());
            int dx;
            int dy;
            for (int k = 0; k < t; k++)
            {
                string prog = Console.ReadLine();
                int p_exp;
                Exp(prog, 0, out dx, out dy, out p_exp);
                x = Mod(x + dx - 1, (int)max) + 1;
                y = Mod(y + dy - 1, (int)max) + 1;

                Console.WriteLine("Case #{0}: {1} {2}", k + 1, x, y);

                x = 1;
                y = 1;
            }
        }
    }
}
