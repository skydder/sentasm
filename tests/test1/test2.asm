@.text
globalize _start

#_start
(move 42 to eax)
move 60 to rax
move 42 to rdi
systemcall
