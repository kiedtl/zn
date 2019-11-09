module main

import os

fn main() {
	// memory buffer
	memory := []byte

	// get input file
	files := os.args

	// read each file
	for file in files  {
		go read(file)
	}
}

fn read(file string) {
}

fn execute(memory []byte) {
	
}
