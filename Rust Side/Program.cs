using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;
using System.Diagnostics;
using Microsoft.Xna.Framework;

namespace Rust_Side
{
    class Program
    {
        [DllImport("rust_math.dll")]
        private static extern Matrix create_look_at(Vector3 pos, Vector3 target, Vector3 up);

        public static Matrix CreateLookAt(Vector3 Pos, Vector3 Target, Vector3 Up)
        {
            return create_look_at(Pos, Target, Up);
        }

        static void Main(string[] args)
        {
            Vector3 cameraPosition = new Vector3(200, 300, 400);
            Vector3 cameraTarget = new Vector3(300, 200, 100);
            Vector3 cameraUp = Vector3.Up;
            List<Matrix> matrices = new List<Matrix>(1_000_000);
            List<long> ticks = new List<long>();
            Stopwatch stopWatch = new Stopwatch();
            for (int j = 0; j < 100; j++)
            {
                stopWatch.Start();
                for (int i = 0; i < 1_000_000; i++)
                {
                    matrices.Add(CreateLookAt(cameraPosition, cameraTarget, cameraUp));
                }
                stopWatch.Stop();
                matrices.Clear();
                ticks.Add(stopWatch.ElapsedTicks);
                stopWatch.Reset();
            }

            // Get the elapsed time as a TimeSpan value.

            Console.WriteLine("RunTime " + ticks.Sum() / 100.0);
            Console.ReadKey();
        }
    }
}
