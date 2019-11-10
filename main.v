module main

import os
import stdlib

fn main() {
	// memory buffer and pointers
	memory		:= []byte
	pointers	:= map[string][]int

	// get input file
	files := os.args

	mut lines := []string

	// read each file
	for file in files  {
		if os.file_exists(file) {
			new_lines := os.read_lines(file, memory, pointers)
			lines << new_lines
		}
	}

	read(lines, memory, variables)
}

fn read(lines []string, memory mut []byte, variables mut map[string][]int) {
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
