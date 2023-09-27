# bf+

## Main goals
- extend bf to be less f*ck'y but still giving the same vibes
- quality of life improvements


## Instructions
Cells are `short unsigned ints` `(u8)` (for now)
#### Original instructions

- `<` moves pointer to the left (wraps around in case of underflow)
- `>` moves pointer to the right (wraps around in case of over flow)
- `+` adds 1 into current cell
- `-` subtracts 1 from current cell
- `.` output current cell into stdout
- `,` 1 byte input from stdio into current cell
- `[` If the current cell is zero, then pointer jumps to the next "`]`"
- `]` If the current cell is non zero, then pointer jumps to the last "`[`"

#### New Instructions

- `'` clear current cell (set 0)
- `?` perform a syscall (look at examples to get better idea) (output parameter into stack)
- `$` put current cell's memmory adress into that cell
- `%` put base memmory address into current cell
- `&` reset pointer (set it to 0)
- `^` push current cell into stack
- `_` pop from stack into current cell
- `*` delete element from top of a stack
- ``` @`{main}` ``` set tape to specified one
- ```#tape msg byte 14 ``` this creates a new tape (if you dont declare any by default you will be on `main` tape which has `byte` cell size and cell_count set to `MEM_SIZE` constant

**One thing to remember depending on your machine data could be in little or in big endian**

### Todo
- [x] Write Nasm ELF 64 compiler
- [x] Optimize compiler (still a lot of bloat)
- [x] Add ability to perform syscalls
- [x] Add macros
- [x] Add str char and numbers literals into bf+
- [ ] If possible rewrite bf+ in itself
- [ ] Add Windows support (somehow)
- [x] Add Push Pop Stack Operation
- [ ] Add bitwise operations
- [x] Add ability to create multiple mem lines and ability to specify their cell size and max len (in cells)
- [x] Add Include Dirs and Libs to compiler
- [ ] Add Ability to call extern functions
- [x] Add #include
- [x] Add #ifdef #ifndef #else #endif 
