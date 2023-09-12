# bf+

## Main goals
- extend bf to be less f*ck'y but still giving the same vibes
- quality of life improvements


## Instructions
Cells are `short unsigned ints` `(u8)`
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
- `?` perform a syscall (look at examples to get better idea)
- `$` put current cell's memmory adress into that cell

**One thing to remember depending on your machine data could be in little or in big endian**

### Todo
- [x] Write Nasm ELF 64 compiler
- [ ] Optimize compiler
- [x] Add ability to perform syscalls
- [ ] Add macros
- [ ] Add str char and numbers literals into bf+
- [ ] Add labels and ability to jmp
- [ ] If possible rewrite bf+ in itself
- [ ] Add Windows support (somehow)