
            section .data
                x dd 0
            section .text
                global _start
            _start:
            line_10:
mov eax, 1
add eax, 1
mov dword [x], eax


line_20:
mov eax, [x]
add eax, 2



