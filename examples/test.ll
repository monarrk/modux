; Header
target triple = "x86_64-unknown-freebsd"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
; Main
define dso_local i32 @main() #0 {
	%a = alloca i32, align 4
	store i32 9, i32* %a, align 4

	ret i32 0
}