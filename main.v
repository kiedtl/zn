module main

import os
import znlib

fn main() {
	// memory buffer and pointers
	mut memory	:= []byte
	mut pointers	:= map[string][]int

	// get input file
	files := os.args

	// read each file
	for file in files  {
		if os.file_exists(file) {
			go read(file, memory, pointers)
		}
	}
}

fn read(file string, memory mut []byte, variables mut map[string][]int) {
	lines := os.read_lines(string)
	for line in lines {
		execute(line, memory, variables)
	}
}

fn execute(line string, memory mut []byte, variables mut map[string][]int) {
	parts := line.split(" ")
	comnd := parts[0]
	match comnd {
		// define a pointer to a space in memory
		'defptr' {
			znlib.defptr(memory, variables, parts)
		}
		// set a cell in memory to a u8 value
		'setval' {
			znlib.setval(memory, variables)
		}
		// print cell/pointer value
		'putbuf' {
			znlib.putbuf(memory, variables)
		}
	}
}
