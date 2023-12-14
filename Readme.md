# Advent of Code Solutions

Multi-year, multi-language repository for my Advent of Code solutions.
Input data is not stored in this repository (as preferred by the the makers of Advent of Code).

Unless stated otherwise, all input is passed in through STDIN and answers will be outputted on STDOUT.
The input should contain a final linebreak and the output will only consist of the answers to part 1 and part 2 on two lines.

## 2023

### Rust

```rust
let rows: Vec<Vec<i64>> = std::iter::successors(Some(top_row), |prev_row| {
  let row = differences(prev_row);
  (!row.iter().all(|&n| n == 0)).then_some(row)
})
```

This year the language of choice will be Rust with solutions compiled as a single project.

- [All Days](/2023/rust/src)

Rust offers the performance associated with low level languages while still retaining the expressiveness of a much higher level language (it has [sum types](https://en.wikipedia.org/wiki/Algebraic_data_type), a feature I can no longer live without). Often I can think in Haskell while writing in Rust if the problem is suited to it. The design of its iterators (and the rest of its std) continue to impress me.

From the [/2023/rust](/2023/rust) directory it can be compiled with `cargo build` or `cargo build --release` (release/optimized binaries execute an order of magnitude faster).
When executing, the day is chosen with a command line arg (as a 0 padded day number):

```bash
./target/release/rust-advent-2023 07 < ../.inputs/07.txt
```

### Brainfuck

```brainfuck
[->+>+<<]>>[-<<+>>]+>+<<[[-]>-<]>[->-<<<<->>>]>
```

My personal crown jewel of this year's Advent.

- [Day 01 Part 1](/2023/brainfuck/day01.bf)

Brainfuck is an esoteric programming language consisting of just 8 operations (`+`, `-`, `>`, `<`, `[`, `]`, `,`, `.`) yet is still somehow Turing complete. It is very difficult to write and, I would contend, almost impossible to read. This leads to me forgetting what precisely the code does as soon as it's written. It's not immediately clear how to do _anything_ in BF (I've never read about writing programs in BF to maintain the mystery) but eventually some constructs start to come together.

I used a custom TUI application where I could write and execute BF step by step (forwards and backwards in time) with breakpoints for jumping large distances. I want to release this TUI application at some point (with a distant dream of backporting it to the Amiga, where Brainfuck originally appeared) but since it is not available, the standard `bf` interpreter can be used.

The solution relies on underflow/overflow (usually the default) and a NULL terminated input:

```bash
(cat ./2023/.inputs/01.txt; printf '\0') | bf ./2023/brainfuck/day01.bf
```

### x64 Assembly

```nasm
call read_number
mov [winning+rbx], al       ; record winning number
inc rbx                     ; increment counter
inc r15                     ; skip trailing space
jmp next_winning
```

Writing in Brainfuck makes assembly feel like the highest level abstraction going.

- [Day 04](/2023/assembly/day04.asm)

Since I've been doing a lot of reverse engineering lately it was nice to actually write a program in assembly to improve my comprehension. Also a nice excuse to finally use `gdb`.

It is written in the Intel syntax and compiled with `nasm`:

```bash
nasm -f elf64 ./2023/asm/day04.asm -o ./target/2023_asm_day_04.o
ld ./target/2023_asm_day_04.o -o ./target/2023_asm_day_04
./target/2023_asm_day_04 < ./2023/.inputs/04.txt
```

### PSQL

```sql
select sum(red * green * blue) from (
  select
    max(case when color = 'red' then amount else null end) as red,
    max(case when color = 'green' then amount else null end) as green,
    max(case when color = 'blue' then amount else null end) as blue
  from games
  group by game_id
);
```

- [Day 02](/2023/psql/day02.sql)

### Haskell

```haskell
derive :: [Int] -> [[Int]]
derive = takeWhile (not . all (== 0)) . iterate derivatives
```

- [Day 09](/2023/haskell/day09.hs)

### Math

- [Day 06](/2023/haskell/day06.png)
