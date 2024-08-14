@.text
globalize _start
globalize main

#_start
call main
#_fini
move rax to rdi
move 60 to rax
systemcall
