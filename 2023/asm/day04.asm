section .bss
    lines resb 32768            ; stdin input
    winning resb 10             ; list of potential winning numbers
    extra_plays resb 1024       ; 256 32bit numbers

section .text
    global _start

_start:
    ; set up some global variables
    mov r12, 0                  ; r12 = part 1 result
    mov r13, 0                  ; r13 = part 2 result
    mov r14, 0                  ; r14 = current game id
    mov r15, lines              ; r15 = next input
    ; read stdin to lines buffer
    mov eax, 0                  ; 'read' syscall
    mov edi, 0                  ; stdin file descriptor
    mov rsi, lines              ; read to lines buffer
    mov edx, 32768              ; max number of bytes to read, can be less
    syscall

game_start:
    ; check if the input is done
    cmp byte [r15], 0
    jz print_results
    inc r14                     ; increment game id
    mov r11, 0                  ; r11 = card score
    mov r10, 0                  ; r10 = matches counter
    mov rbx, 0                  ; rbx = winning numbers counter
    jmp skip_game_id            ; begin skipping game id input

skip_game_id_loop:
    inc r15
skip_game_id:
    ; check for the id delimiter
    cmp byte [r15], ':'
    jnz skip_game_id_loop
    inc r15                     ; skip semicolon
    inc r15                     ; skip leading space

next_winning:
    ; check for delimiter between winning and chosen numbers
    cmp byte [r15], '|'
    jz match_chosen

    call read_number
    mov [winning+rbx], al       ; record winning number
    inc rbx                     ; increment counter
    inc r15                     ; skip trailing space
    jmp next_winning

match_chosen:
    inc r15                     ; skip pipe
next_chosen:
    ; check for line break, game delimiter
    cmp byte [r15], 10
    jz post_card

    inc r15                     ; skip leading space
    call read_number
    mov r8, 0                   ; r8 = winning number loop index
compare_numbers:
    ; check if the chosen number is not a winning number
    cmp [winning+r8], al
    jnz compare_next

    inc r10                     ; chosen card was matched
    ; check if this is the first score for the card
    cmp r11, 0
    jz first_score
    ; double the card's score
    mov rdi, 2
    imul r11, rdi
    jmp compare_next
first_score:
    mov r11, 1
compare_next:
    ; check if all winning numbers have been iterated over
    inc r8
    cmp r8, rbx
    jz next_chosen
    jmp compare_numbers

post_card:
    inc r15                     ; skip line break
    add r12, r11                ; add card score to result 1
    ; use current game id to 32bit index into extra_plays
    lea rax, [extra_plays + (r14*4)]
    ; inc the play counter for current game and add it to result 2
    xor rbx, rbx
    mov ebx, [rax]
    inc ebx
    add r13, rbx
    ; skip to next card if this one had no matches
    cmp r10, 0
    jz game_start

    add rax, 4                  ; move the extra_plays offset to next game
add_extra_plays:
    add [rax], ebx              ; add extra plays to subsequent game
    add rax, 4                  ; move offset to next game
    ; check if all next games have been iterated through
    dec r10
    cmp r10, 0
    jnz add_extra_plays
    jmp game_start

print_results:
    mov rdi, r12
    call print_u32
    mov rdi, r13
    call print_u32
    jmp exit

; reads the next number from global input (r15)
; rax = returned number
read_number:
    mov rax, 0
    ; single digit numbers start with a space
    cmp byte [r15], ' '
    jz second_digit
    mov al, [r15]               ; read character digit
    sub al, 48                  ; convert to int digit
    ; shift first digit left (in decimal)
    mov dil, 10
    imul dil
second_digit:
    inc r15
    mov dil, [r15]
    inc r15
    sub dil, 48
    add al, dil
    ret

; prints a u32 to stdout
; rdi = number to print
print_u32:
    mov rax, rdi                ; rax = quotient
    lea rsi, [rsp - 1]          ; rsi = message buffer
    mov byte [rsi], 10          ; message ends with a line break
    dec rsi
    mov rcx, 1                  ; rcx = message length
print_loop:
    ; div/rem the number by 10 to get the last digit
    xor rdx, rdx
    mov r8, 10
    div r8
    ; move digit into ascii range, add it to message
    add dl, 48
    mov [rsi], dl
    dec rsi
    inc rcx
    ; loop if the quotient is not 0
    cmp rax, 0
    jnz print_loop
    ; write to stdout
    mov rax, 1                  ; 'write' syscall
    mov rdi, 1                  ; stdout file descriptor
    inc rsi                     ; undo unwanted dec (rsi is write's buffer)
    mov rdx, rcx                ; message length
    syscall

    ret

exit:
    mov rax, 60
    xor rdi, rdi
    syscall
