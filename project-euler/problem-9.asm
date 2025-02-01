start:
    mov cx, 1
loop_i:
    mov dx, cx
    inc dx
loop_j:
    mov ax, cx 
    mul ax
    mov bx, ax
    mov ax, dx
    mul ax
    sub ax, bx 
    mov si, ax
    ; j * j - i * i = a
    mov ax, cx
    mul dx
    mul 2
    mov di, ax
    ; i * j * 2     = b
    mov ax, cx 
    mul ax
    mov bx, ax
    mov ax, dx
    mul ax
    add ax, bx
    mov bx, ax
    ; j * j + i * i = c
    add ax, si
    add ax, di
    cmp ax, 1000
    je end
loop_j_end:
    inc dx
    cmp dx, 32
    jl  loop_j
loop_i_end:
    inc cx
    cmp cx, 32
    jl  loop_i
end:
    mov ax, bx
    mul si
    mul di
