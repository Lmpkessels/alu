# OR8MULTI - Hardcoded

Build a hardcoded OR8MULTI because I was reading Nand2Tetris, and diving deep into
Rust which communicates at the lowest level with computer components.

## What it does:

- Takes in an argument of 8 arrays of 8 bits.
- Gets each index, and applies OR logic.
- Returns 1 Vector with the result per index after applying or.

## What I learned:

- Range can be used to get each index position individually.
- Double slice can be used to communicate the index precise.
- Visual thinking, in columns and rows.
- That it can take up to 7 hours or more to figure our 50 lines of code.
- That it would've been better if I used parameters instead of a nested array.

## Next

Rewrite, using cleaner controll flow. Then build `and8multi` `xor8multi` `not8`
etc.

## Update on 8MULTI

Used loops to make the code shorter, and more efficient.

## What I learned:

- That what you put in the loop gets looped that ammound of time, so if a println!
  contains the value of `"Hello"`, and the loop has a range of 8 for example then
  `"Hello"` is printed 8 times.
