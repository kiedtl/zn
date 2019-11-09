// builtin ZN functions, like defptr or putbuf
module znlib

fn die(reason string) {
	println("ERROR: $reason")
	exit(1)
}

fn defptr(
	memory mut []byte,
	variables mut map[string][]int,
	command []string
)
{
	ptr_name := command[1]
	start_addr := command[2]
	end_addr := command[3]

	//variable names may not begin with %, &, or *.
	if ptr_name.starts_with('%') ||
		ptr_name.starts_with('&') ||
		ptr_name.starts_with('*') 
	{
		die("ptr names may not start with `%`, `&`, or `*`!")
	}

	// ensure addresses are valid
	if ! start_addr.starts_with('%') || ! end_addr.ends_with('%') {
		die("invalid addresses: ${start_addr} and/or ${end_addr}")
	}

	variables[ptr_name] = [start_addr.replace('%', '').to_i(), end_addr.replace('%', '').to_i()
