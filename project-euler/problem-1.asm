mov ax, 0
mov cx, 0
mov dx, 0
loop:
    mov ax, cx
    mod 3
    je  to_add
    mov ax, cx
    mod 5
    je  to_add
to_inc:
    inc cx
    cmp cx, 1000
    jl loop
    jmp end
to_add:
    add dx, cx
    jmp to_inc
end:
