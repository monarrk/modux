rule Fn_define_puts
{
        meta:
	ir = "declare i32 @puts(i8* nocapture) nounwind"
	location = "header"

	strings:
	$ = "init puts"

	condition:
	any of them
}

rule Fn_puts_char
{
        meta:
        ir = "call i32 @puts(i8* %#)"
        location = "main"
        starts = "("
        ends = ")"

        strings:
         $a = /puts_char \([a-zA-Z]*\)/

        condition:
        $a and Fn_define_puts
}

rule Fn_puts_str
{
        meta:
        ir = "%$ = alloca i8*, align 8\n\tstore i8* getelementptr inbounds ([# x i8], [# x i8]* @.#, i64 0, i64 0), i8** %^1, align 8\n\t%$ = load i8*, i8** %^2, align 8\n\tcall i32 @puts(i8* %^1)"
        location = "main"
        starts = "{:{:("
        ends = "}:}:)"

        strings:
        $a = /puts \([a-zA-Z]*\)\{[0-9]*\}/

        condition:
        $a and Fn_define_puts
}
