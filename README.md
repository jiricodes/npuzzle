# npuzzle

This is a 42 curriculum project oriented around A* and relvant heuristics.

The goal is to find solution to given [N-puzzle](https://en.wikipedia.org/wiki/15_puzzle) (*[Taquin](https://en.wikipedia.org/wiki/Jeu_de_taquin)* in French) state.

## Notes / Todo
### Organisation
- [ ] Launcher
    - [ ] pars args
    - [ ] handle setup
    - [ ] handle solver init
    - [ ] handle output format
- [ ] Input Handler
    - [ ] from raw to vec<vec<T>>
    - [ ] input validation
- [ ] Generator
    - [x] generates custom states
        - [ ] test well!
    - [x] generates expected solution
- [ ] Solver
    - [ ] multiple implementations based on the algo
    - [ ] Heur as a separate unit?
- [ ] Stats Handler
    - [ ] Should keep track of required stats at least
    - [ ] Think of interesting bonuses
- [ ] Solution
    - [ ] Return type from `Solver`
    - Subject:
        - [ ] Complexity in time
        - [ ] Complexity in space
        - [ ] *path*
        - [ ] path.len()
        - [ ] Solvability report -> investigate parity check
- [ ] Puzzle Types
    - [x] Snail (Subject)
    - [ ] Line by line and Empty at 0,0
    - [ ] Line by line and Empty at n-1,n-1
### A*
- [ ] Standard A* (subject)
    - [ ] uniform-cost 
    - [ ] greedy (bonus)
- [ ] Research if IDA* would have any performance improvements
- [ ] Wighted A* - explore
- [ ] Bidirectional A*
- [ ] parallel compute - explore


### Heuristics
- [ ] Manhattan distance
- [ ] Hamming distance

### Others
- [ ] benchmarks
- [ ] stats
- [ ] parity check for solvability - explore

### Bonuses
- [ ] different types of solution other than the snail one

