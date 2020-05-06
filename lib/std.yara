include "io.yara"

rule Def_i32
{
	meta:
	ir = "%# = alloca i32, align 4"
	location = "main"
	starts = "("
	ends = ")"

	strings:
	$ = /var \([a-zA-Z]*\) i32/

	condition:
	any of them
}

rule Mut_i32
{
	meta:
	ir = "store i32 #, i32* %#, align 4"
	location = "main"
	starts = "[:("
	ends = "]:)"

	strings:
	$store = /i32 \[[0-9]*\] -> \([a-zA-Z]*\)/

	condition:
	$store and Def_i32
}

rule Def_char
{
	meta:
	ir = "%# = alloca i8, align 1"
	location = "main"
	starts = "("
	ends = ")"

	strings:
	$ = /var \([a-zA-Z]*\) i8/

	condition:
	any of them
}

rule Mut_char
{
	meta:
	ir = "store i8 #, i8* %#, align 1"
	location = "main"
	starts = "[:("
	ends = "]:)"

	strings:
	$store = /i8 \[[0-9]*\] -> \([a-zA-Z]*\)/

	condition:
	$store and Def_char
}

rule Def_str
{
	meta:
	ir = "@.# = private unnamed_addr constant [# x i8] c\"#\", align 1"
	location = "header"
	start = "(:{:["
	ends = "):}:]"

	strings:
	$ = /global str \[[a-zA-Z0-9\s]*\]\{[0-9]*\} -> \([a-zA-Z]*\)/

	condition:
	any of them
}

rule Loop_start
{
	meta:
	ir = "br label %#\n#:"
	location = "main"
	start = "(:("
	end = "):)"

	strings:
	$ = /loop \([a-zA-Z]*\) do/

	condition:
	any of them
}

rule Loop_end
{
	meta:
	ir = "br label %#"
	location = "main"
	start = "("
	end = ")"

	strings:
	$ = /end \([a-zA-Z]*\)/

	condition:
	any of them
}
