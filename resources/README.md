# Resources

## Unsorted
- [not the shorthest but max n3 moves](https://cseweb.ucsd.edu//~ccalabro/essays/15_puzzle.pdf)
- [princeton 8puzzle project](https://www.cs.princeton.edu/courses/archive/spr10/cos226/assignments/8puzzle.html)
- [IDA* in C](https://rosettacode.org/wiki/15_puzzle_solver#IDA.2A)
- [Solving 15 puzzle with literate programming](https://kenogo.org/literate_programming/15_puzzle_solver.pdf)
- [Suboptimal Map Pathfinding Algorithms](https://www.movingai.com/SAS/SUB/)
- [A* with goal bounding](http://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter22_Faster_A_Star_with_Goal_Bounding.pdf)
- [Mechanical approach / specialised](https://www.kopf.com.br/kaplof/how-to-solve-any-slide-puzzle-regardless-of-its-size/)
    - Multiple ways exist actually
        - https://www.youtube.com/watch?v=NXRIrP1k4dE&ab_channel=StevenBurrell
        - https://webdocs.cs.ualberta.ca/~hayward/396/jem/tile.html
    - could help with pruning perhaps

## [npuzzle.en.pdf](npuzzle.en.pdf)
Given subject describing the problem, limitations, requirements and goals of this project.

## [npuzzle_eval.md](npuzzle_eval.md)
Evaluation form as of June 2021 at Hive Helsinki

## [res_npuzzle-gen.py](res_npuzzle-gen.py)
Project supplied npuzzle generator.

## Red Blob Games by Amit Patel's
There are plenty resources regarding A* and other algorithms at Amit Patel's [blog](http://www-cs-students.stanford.edu/~amitp/) and Red Blob Games [website](https://www.redblobgames.com/). Here is a list of some relevant ones for this project that have been used in the research phase.

### [Introduction to A *](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
Very beginner friendly practical and visual guide to A* through graph representation, BFS and Dijkstra.

### [Variants of A*](http://theory.stanford.edu/~amitp/GameProgramming/Variations.html)
- **IDA\***
    - ID algorithms tend to increase computation time while reducing memory requirements. In map pathfinding, however, the “nodes” are very small―they are simply coordinates.
- **Weighted A\***
    - multiply heuristic by some constant to make the A* faster
    - [when it doesn't work](https://www.cs.unh.edu/~ruml/papers/wted-astar-socs-12.pdf)
- **Bandwidth search**
    - This variation assumes that *h* is an overestimate, but that it doesn’t overestimate by more than some number *e*. If this is the case in your search, then the path you get will have a cost that doesn’t exceed the best path’s cost by more than *e*.
    - Whenever h+d is greater then the true cost of the path (for some d), you can drop any node that has an f value that’s at least e+d higher than the f value of the best node in OPEN.
- **Bidirectional search**
    - two searches in "parallel" front-to-back and back-to-front, stops when they meet
    - different implementation versions - front-to-front, retargeting etc.
- **Jump Point Search**
    - When considering children of the current node for possible inclusion in the OPEN set, Jump Point Search skips ahead to faraway nodes that are visible from the current node.
    - worth considering for larger Npuzzles perhaps, where tiles may travel in straight lines more often
- Not relevant to the project really, just for sake of notekeeping
    - **Dynamic A* and Lifelong Planning A***
        - versions that support map changes and correct potential errors from previous A* path
        - not feasible whaen many moving units are present, due to it's memory demands
    - **Theta\***
        - Theta*, a variant of A* that runs on square grids, to find paths that don’t strictly follow the grid. When building parent pointers, Theta* will point directly to an ancestor if there’s a line of sight to that node, and skips the nodes in between.
        - perhaps worth using together with Suboptimal Pathfinding Algos (e.g. inadmissible heuristics)
        - [Block A*](https://webdocs.cs.ualberta.ca/~holte/Publications/aaai11PeterYapFinal.pdf)
        - http://www.aaai.org/ocs/index.php/AIIDE/AIIDE11/paper/download/4055/4436
    - **Flow-field pathfinding**
        - https://www.redblobgames.com/pathfinding/tower-defense/
### [Approximate Heuristics](http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#approximate-heuristics)
Worth to explore

### [Finding Optimal Solutions to the Twenty-Four Puzzle](https://courses.cs.washington.edu/courses/csep573/10wi/korf96.pdf)
-  The state space for
    - 8-puzzle - 10<sup>5</sup> nodes
    - 15-puzzle - 10<sup>13</sup> nodes
    - 24-puzzle - 10<sup>25</sup> nodes
    - num_states = (width * height)! / 2
- Used **IDA with Manhattan distance heuristics** (note: sum for all tiles) to solve 15-puzzle. Each path is explored until  a node *n* is reached where the number of moves from the initial state , *g(n)*, plus the heuristic estimate of the number of moves necessary to reach the goal state, *h(n)*, exceeds a treshold for that iteration. The threshold for the first iteration is the heuristic estimate for the initial state, and the treshold for each succeeding iteration is the minimum total cost, *f(n) = g(n) + h(n)*, of all nodes on the frontier of the previous iteration.
- **Linear Conflict Heuristic**
    - in case of two tiles are in reversed position (e.g. 2, 1) than desired, two moves can be added to Manhattan distance without breaking admissibility
    - in case of three tiles in reverse order (e.g. 3, 2, 1), 4 moves can be added without breaking admissibility
    - additionally a tile can be in conflict in row and column at the same time - both sets of extra moves can be added without breaking admissibility
    - efficiently computing this heuristic involves precomputing and storing all possible permutations of tiles in a row or column, and incrementally computing the heuristic value of a child from that of its parent
- **Last Moves Heuristic**
    - when setting empty to it's position, some of it's neighbours needs to be in it's position. If not, we can add 2 moves to Manhattan without breaking adm. (**note: not sure if this is applicable in our case of the snail like solution, where there are 4 neighbours instead of 2 in common cases**)
    - could be extended to two last moves
    - three last do no seem cost effective
    - check if Linear Conflic didn't add moves for the same reason
- **whenever using more than one heuristic, we must compute their interactions to mainatin admissibility**
- bidirectional search - specifically **perimeter search** reported efficiency improvement with manhattan heur. However not sure what would be interaction with above stated heurs
- **Corner-Tiles Heuristic**
    - when second last tile in a row is it's position, and last is not, it temporarily needs to leave its position
    - adding 2 moves doesn't break adm.
    - check for previous heurs interaction (esp linear conflict)
- **Pruning Duplicate Nodes**
    - pruning moves that would cancel each other (e.g. Left+down and right+up afterwards)
    - last operator as the state of afinite-state machine
    - ideally only one path to each node in the grid
    - this can be done by first all Left or all Right, folled by single turn, then all Up ir all Down.
    - these rules can also be enforced bu a five-state finite-state machine
    - check the paper for detailed description

### [Solving the (n<sup>2</sup> - 1)-Puzzle with (8/3)n<sup>3</sup> Expected Moves](https://www.mdpi.com/1999-4893/8/3/459/pdf)
- greedy algo analysis

## [Common Misconceptions Concerning Heuristic Search](https://aaai.org/ocs/index.php/SOCS/SOCS10/paper/viewFile/2073/2500)
