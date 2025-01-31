start:
    mov ax, 1
    mov bx, 20
loop:
    mov cx, 1
check:
    mov ax, bx
    mod cx
    cmp ax, 0
    je check_tail
    add bx, 20
    jmp loop
check_tail:
    inc cx
    cmp cx, 20
    jg  end
    jmp check
end:
