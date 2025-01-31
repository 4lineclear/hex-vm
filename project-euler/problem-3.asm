    mov cx, 600851475143
    mov bx, 2
loop:
    mov ax, bx
    mul ax
    cmp ax, cx
    jge end
    mov ax, cx
    mod bx
    cmp ax, 0
    jne be
    mov ax, cx
    div bx
    mov cx, ax
    jne loop
be:
    inc bx
    jmp loop
end:
