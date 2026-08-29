# How to read this book

Read it in order, with a terminal open. The chapters are cumulative:
each one is written assuming you typed the last one's programs, and the
exercises are where the material stops being something you read.

Every program in this book was executed by CI against the toolchain
version printed in the [colophon](../back/colophon.md): the greetings,
the failures, the traps, the diagnostics, the exercise solutions. Tool
output is pasted from those runs and never retyped. If a line on the
page differs from a line in your terminal, one of two things is true:
your toolchain is not the one in the colophon, or the book has a bug.
Both are worth reporting, and the colophon is how you tell them apart.

## The on-ramp for your background

The book has one path, but the first two parts land differently
depending on what you already do all day. Wherever a section can be
read as an argument with a language you know, it is set off in a box
that names it. Skipping those boxes costs you nothing technical.

**Coming from Python.** You will be at home in part 1 and ambushed in
part 2. Chapter 2 is the important one: wolf's strings count bytes and
say so. Read chapter 3's section on checked arithmetic slowly, because
it is where an integer stops being a mathematical integer, and expect
chapter 7 to be a genuinely new idea rather than a syntax to learn.

**Coming from Go.** Part 1 will feel like a shorter Go, and the two
places it is not are these: errors are values *in the
type*, so chapter 6 replaces `if err != nil` with a signature, and
there is no garbage collector, so part 2 is not optional. Chapter 10
onward is where the languages meet again, and where the differences
between a goroutine and a wolf task are argued in full.

**Coming from Rust.** You already believe most of part 2's conclusions.
Read it anyway for the part you will not believe: there are no lifetime
annotations, and chapter 8's regions are why. Skim part 1 for the
syntax, slow down at chapter 7's granularity rules, and go straight to
chapter 8 if you are impatient.

**Coming from C.** Part 1 is short work. What is new is that the
machine's mistakes are named and stopped rather than left to you:
chapter 3's traps, chapter 2's checked slices. Chapter 9 is the one you
came for: the C membrane, and exactly how much of your existing code you
have to trust.

## What the parts do

Part 1 makes you dangerous in one file: values, functions, collections,
errors. Part 2 answers "who owns this" and is the part the language
exists for. Part 3 is concurrency, part 4 is the machine and the
ecosystem, and part 5 builds programs, spending what the earlier parts
taught and introducing nothing new. By the time you reach it the
language is one you already know, and none of the projects is a trick.
Exercises end the section whose material they exercise, and
every one of them has a solution in the back. The solo project publishes
checkpoints instead of answers, and it is the only page in the book that
withholds one.

## If you are in a hurry

Chapter 1, chapter 3, chapter 6, chapter 7. That is the shortest path
to writing wolf that a reviewer would accept, and it is about an
evening. Come back for chapter 2 the first time a string surprises you,
which will be soon.
