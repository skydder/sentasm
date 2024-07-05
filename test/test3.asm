main:
move 0 to eax
move 1 to ebx
loop:
add ebx to eax
add 1 to ebx
compare ebx to 10
jump to loop if <=
return