@.text
globalize _start
extern main

#_start
call main
#_fini
move rax to rdi
move 60 to rax
systemcall
