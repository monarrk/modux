# Modux
> *There is no language*

An experimental language with extremely modular syntax.

### Building
Run `cargo build`.

### Examples
Because there is *technically* no syntax, this example is not really an example of the language itself but of the standard syntax. That being said, here's a 'hello world':

```
; Anything that isn't regular syntax is a comment!

; Add the `puts` function to the header
init puts

; Strings have a static length defined with {[0-9]*}
global str [hello world]{11} -> (hello)
puts (hello){11}
```

### Defining syntax
Syntax is defined using [YARA](https://virustotal.github.io/yara/). To define a chunk of syntax, create a new rule, with strings for each pattern that represents that expression. Use metadata to define the LLVM IR that will be omitted, using `#` for placeholders for characters which will get sliced out of the strings. If you do use placeholders, define the starts and ends of the slice like so:

```
metadata:
	ir = "; test # and #
	starts = "0:3"
	ends = "1:4"
```

This will slice the string like `string[0..1]` and `string[3..4]`.

### Examples
Go to `examples/`. In here, you will notice two files: `test.mx` and `rules.yara`. `rules.yara` contains all the rules that will be used to parse the file, and all this one does is import the standard syntax. `test.mx` is the code we will actually compile. When in this directory, you can simply run `modux test.mx`, which will scan the `rules.yara` file and compile to LLVM IR. If you wish to import a custom rules file, you can do so with the `-r` flag: `modux -r custom_rules.yara test.mx`.

### Disclaimers
- This language is a terrible idea
- This language should not be used for anything
- This language is a result of procrastination and adderrall
- Please don't look at my rust code; it is irredeemable
