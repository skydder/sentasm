main:
add 1 to eax
substract 1 from @[1](disp)("@[memory location]" indicates memory locaion )
substract 1 from @[ax](base)
substract 1 from @[ax+rax+1](base+idx+disp)
substract 1 from @[ax+1](base+disp)
substract 1 from @[ax+rax*1+1](base+idx*scl+disp)
substract 1 from @[ax*1+1](idx*scl+disp)
substract 1 from @[ax+rax*1](base+idx*scl)
substract 1 from @[ax+rax](base+idx)

substract 1 from @[-1](disp)
substract 1 from @[ax+rax-1](base+idx+disp)
substract 1 from @[ax-1](base+disp)
substract 1 from @[ax+rax*1-1](base+idx*scl+disp)
substract 1 from @[ax   *  1-1]

multiply eax by ebx
divide eax
move 1 to eax
systemcall
return
leave
halt
no-operation
jump to main
add xmm0 to xmm1 as double-precision-float
and eax with eax
or eax with eax
xor eax with eax
shift-right eax by 8
not eax
call printf
jump to main if <
(this is comment)