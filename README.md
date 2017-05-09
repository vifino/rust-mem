# mem

Rust mem crate for emulators.

## What?

A Rust crate that defines a memory trait, which allows rather simple access to blocks of memory.

## Why?

This is mainly for emulators, made for me. I hope some other people find this useful as well.

Why a trait instead of just a simple memory implementation? Because "memory middleware" sounds awesome.

Imagine a single interface that allows you to access local memory, memory over the network, disks, device files and regular files as if they are same thing.

Imagine an emulator using that for not only it's RAM, but also, say, a floppy for a floppy contoller. You could have virtual floppies temporarily stored in RAM, persistantly stored on disk, etc...

Now, this might not sound too useful to you, but you can do a lot with this.

## Tell me more.

The main trait `MemoryBlock` is very simplistic, but should allow a great deal of flexibility.

This crate ships with some implementations to get you started, in the `std_impls` module. Notably `MemVector` which stores the RAM content in a `Vec`. This should be fine for RAM usage.