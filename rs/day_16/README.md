# [--- Day 16: Reindeer Maze ---](https://adventofcode.com/2024/day/16)

It's time again for the [Reindeer Olympics](/2015/day/14)! This year, the big event is the **Reindeer Maze**, where the Reindeer compete for the **lowest score**.

You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?

The Reindeer start on the Start Tile (marked `S`) facing **East** and need to reach the End Tile (marked `E`). They can move forward one tile at a time (increasing their score by `1` point), but never into a wall (`#`). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by `1000` points).

To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:

<pre><code>###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
</code></pre>

There are many paths through this maze, but taking any of the best paths would incur a score of only **`7036`**. This can be achieved by taking a total of `36` steps forward and turning 90 degrees a total of `7` times:

<pre><code>
###############
#.......#....<b>E</b>#
#.#.###.#.###<b>^</b>#
#.....#.#...#<b>^</b>#
#.###.#####.#<b>^</b>#
#.#.#.......#<b>^</b>#
#.#.#####.###<b>^</b>#
#..<b>></b><b>></b><b>></b><b>></b><b>></b><b>></b><b>></b><b>></b><b>v</b>#<b>^</b>#
###<b>^</b>#.#####<b>v</b>#<b>^</b>#
#<b>></b><b>></b><b>^</b>#.....#<b>v</b>#<b>^</b>#
#<b>^</b>#.#.###.#<b>v</b>#<b>^</b>#
#<b>^</b>....#...#<b>v</b>#<b>^</b>#
#<b>^</b>###.#.#.#<b>v</b>#<b>^</b>#
#S..#.....#<b>></b><b>></b><b>^</b>#
###############
</code></pre>

Here's a second example:

<pre><code>#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
</code></pre>

In this maze, the best paths cost **`11048`** points; following one such path would look like this:

<pre><code>#################
#...#...#...#..<b>E</b>#
#.#.#.#.#.#.#.#<b>^</b>#
#.#.#.#...#...#<b>^</b>#
#.#.#.#.###.#.#<b>^</b>#
#<b>></b><b>></b><b>v</b>#.#.#.....#<b>^</b>#
#<b>^</b>#<b>v</b>#.#.#.#####<b>^</b>#
#<b>^</b>#<b>v</b>..#.#.#<b>></b><b>></b><b>></b><b>></b><b>^</b>#
#<b>^</b>#<b>v</b>#####.#<b>^</b>###.#
#<b>^</b>#<b>v</b>#..<b>></b><b>></b><b>></b><b>></b><b>^</b>#...#
#<b>^</b>#<b>v</b>###<b>^</b>#####.###
#<b>^</b>#<b>v</b>#<b>></b><b>></b><b>^</b>#.....#.#
#<b>^</b>#<b>v</b>#<b>^</b>#####.###.#
#<b>^</b>#<b>v</b>#<b>^</b>........#.#
#<b>^</b>#<b>v</b>#<b>^</b>#########.#
#S#<b>></b><b>></b><b>^</b>..........#
#################
</code></pre>

Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.

Analyze your map carefully. **What is the lowest score a Reindeer could possibly get?**

## --- Part Two ---

Now that you know what the best paths look like, you can figure out the best spot to sit.

Every non-wall tile (`S`, `.`, or `E`) is equipped with places to sit along the edges of the tile. While determining which of these tiles would be the best spot to sit depends on a whole bunch of factors (how comfortable the seats are, how far away the bathrooms are, whether there's a pillar blocking your view, etc.), the most important factor is **whether the tile is on one of the best paths through the maze**. If you sit somewhere else, you'd miss all the action!

So, you'll need to determine which tiles are part of **any** best path through the maze, including the `S` and `E` tiles.

In the first example, there are **`45`** tiles (marked `O`) that are part of at least one of the various best paths through the maze:

<pre><code>###############
#.......#....<b>O</b>#
#.#.###.#.###<b>O</b>#
#.....#.#...#<b>O</b>#
#.###.#####.#<b>O</b>#
#.#.#.......#<b>O</b>#
#.#.#####.###<b>O</b>#
#..<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#<b>O</b>#
###<b>O</b>#<b>O</b>#####<b>O</b>#<b>O</b>#
#<b>O</b><b>O</b><b>O</b>#<b>O</b>....#<b>O</b>#<b>O</b>#
#<b>O</b>#<b>O</b>#<b>O</b>###.#<b>O</b>#<b>O</b>#
#<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#...#<b>O</b>#<b>O</b>#
#<b>O</b>###.#.#.#<b>O</b>#<b>O</b>#
#<b>O</b>..#.....#<b>O</b><b>O</b><b>O</b>#
###############
</code></pre>

In the second example, there are **`64`** tiles that are part of at least one of the best paths:

<pre><code>#################
#...#...#...#..<b>O</b>#
#.#.#.#.#.#.#.#<b>O</b>#
#.#.#.#...#...#<b>O</b>#
#.#.#.#.###.#.#<b>O</b>#
#<b>O</b><b>O</b><b>O</b>#.#.#.....#<b>O</b>#
#<b>O</b>#<b>O</b>#.#.#.#####<b>O</b>#
#<b>O</b>#<b>O</b>..#.#.#<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#
#<b>O</b>#<b>O</b>#####.#<b>O</b>###<b>O</b>#
#<b>O</b>#<b>O</b>#..<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#<b>O</b><b>O</b><b>O</b>#
#<b>O</b>#<b>O</b>###<b>O</b>#####<b>O</b>###
#<b>O</b>#<b>O</b>#<b>O</b><b>O</b><b>O</b>#..<b>O</b><b>O</b><b>O</b>#.#
#<b>O</b>#<b>O</b>#<b>O</b>#####<b>O</b>###.#
#<b>O</b>#<b>O</b>#<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>..#.#
#<b>O</b>#<b>O</b>#<b>O</b>#########.#
#<b>O</b>#<b>O</b><b>O</b><b>O</b>..........#
#################
</code></pre>

Analyze your map further. **How many tiles are part of at least one of the best paths through the maze?**
