
@.text
globalize plus
globalize _start

#plus
add rdi to rsi
move rsi to rax
return

#_start
move 3 to rdi
move 4 to rsi
call plus
move rax to rdi
move 60 to rax
systemcall
