.section .text.entrypoint
.global _entrypoint
_entrypoint:
    la sp, _initial_stack_pointer
    # Store player vars in last registers
    # Don't use these for anything else!
    # uhp
    li s11, 50
    # mhp
    li s10, 100
    # upot
    li s9, 3

    la a0, welcome_string
    call putstring
    # todo: LOOP
.Lmaingame_loop:
    li a0, '\n'
    call sbi_console_putchar

    la a0, user_info_s_one
    call putstring

    li a0, ' '
    call sbi_console_putchar

    mv a0, s11
    call putint

    li a0, ' '
    call sbi_console_putchar

    la a0, user_info_s_two
    call putstring

    li a0, ' '
    call sbi_console_putchar

    mv a0, s9
    call putint

    li a0, ' '
    call sbi_console_putchar

    la a0, user_info_s_three
    call putstring

    li a0, ' '
    call sbi_console_putchar

    mv a0, s10
    call putint

    li a0, ' '
    call sbi_console_putchar

    la a0, user_info_s_four
    call putstring

    li a0, ' '
    call sbi_console_putchar
.Lgetchar_loop:
    call sbi_console_getchar
    blt a0, x0, .Lgetchar_loop
    li t1, 'a'
    li t2, 'p'
    beq t1, a0, .Ldoattack_jump
    beq t2, a0, .Ldopotion_jump
.Ldoattack_jump:
    li t0, 1
    la a0, user_attacks
    call putstring
    addi s10, s10, -10
    call .Lenemyattack_jump

.Ldopotion_jump:
    la a0, user_potions
    call putstring
    # TODO: don't add greater than 50
    addi s11, s11, 30
    # call .Lenemyattack_jump

.Lenemyattack_jump:
    # enemy attacks
    la a0, enemy_attacks
    call putstring
    addi s11, s11, -10

    blt s11, t0, .Lendgame_jump # user dies
    blt s10, t0, .Lendgame_jump # enemy dies
    bge s11, t0, .Lmaingame_loop # user is still alive

.Lendgame_jump:
    blt s11, t0, .Luserdies # user dies
    bge s11, t0, .Luserlives # user is still alive
.Luserdies:
    la a0, user_dies
    call putstring
    bge s10, t0, .Lshutdown_jump # enemy doesnt die
    la a0, user_and_enemy_die
    call putstring
    call .Lshutdown_jump
.Luserlives:
    la a0, user_wins_without_dying
    call putstring

.Lshutdown_jump:
    call sbi_shutdown
.data
welcome_string:
.string "Welcome to Fight!"

user_info_s_one:
# There's some issues with ending with a space in this string..
.string "You have:"

user_info_s_two:
# There's some issues with starting with a space in this string..
.string "/ 50 HP. Potions:"

user_info_s_three:
.string "\nEnemy has:"

user_info_s_four:
.string "/ 100 HP\nWhat will you do? (a)ttack, or (p)otion?"

user_attacks:
.string "\nYou attack, dealing 10 damage!\n"

enemy_attacks:
.string "\nThe enemy attacks, dealing 10 damage!\n"

user_potions:
.string "\nYou drink a potion, healing 30 hitpoints.\n"

user_dies:
.string "You died.\n"

user_and_enemy_die:
.string "The fact that your enemy died too is small consolation for you.\n"

user_wins_without_dying:
.string "You didn't die!\n"

.text









enemyattack:
    la a0, enemy_attacks
    call putstring
    addi s11, s11, -10
    # blt s11, t0, .Lendgame_jump
    # blt s10, t0, .Lendgame_jump
    # bge s11, t0, .Lmaingame_loop


putspace:
    li a0, ' '
    call sbi_console_putchar
    ret



# sp and s1 and s2
# V
# ??????????s0s0s1s1s2s2rara

# VARIABLES!
# stack reg var why
# 4     ra: we need to save this too
# 4     s0: n: the input number
# 4     s1: array_of_digits: the beginning of the digits
# 4     s2: next_digit_to_output: a pointer into the array of digits
# -     a0: ch: a character to output
# -     t0: temp char code to output
# 10        the array
# 4 + 4 + 4 + 4 + 10 = 26

putint:
    # Reserve 26 bytes of stack
    addi sp, sp, -26
    sw s0, 10(sp)
    sw s1, 14(sp)
    sw s2, 18(sp)
    sw ra, 22(sp)
    # now we can do the actual function
    mv s0, a0 # s0: n: the input number
    mv s1, sp # s1: array_of_digits
    mv s2, sp # s2: next_digit_to_output
    li t1, 10 # t1 gets to be 10. It's ten now.
.Lgetcharcode_loop:
    # let temp = character_code_of('0') + (n % 10);
    rem t0, s0, t1
    addi t0, t0, '0'
    # next_digit_to_output[0] = temp;
    sb t0, 0(s2)
    # next_digit_to_output += 1;
    addi s2, s2, 1
    # n = n / 10;
    div s0, s0, t1
    # ... while(n > 0)
    bgt s0, x0, .Lgetcharcode_loop
    # oh! we're doing a(nother) loop! -SB
.Lputintsupersickdothingsloop:
    # next_digit_to_output -= 1;
    addi s2, s2, -1
    # ch = next_digit_to_output[0];
    lb a0, 0(s2)
    # sbi_console_putchar(ch);
    call sbi_console_putchar
    # while (next_digit_to_output > array_of_digits)
    bgt s2, s1, .Lputintsupersickdothingsloop
    # all done with the actual function
    lw s0, 10(sp)
    lw s1, 14(sp)
    lw s2, 18(sp)
    lw ra, 22(sp)
    addi sp, sp, 26
    ret
    

putstring:
    # Reserve 8 bytes on the stack.
    addi sp, sp, -8
    # byte 0-3 on the stack: 
    sw s0, 0(sp)
    # byte 4-7 on the stack: our return address being saved for later
    sw ra, 4(sp)
    # put a0 (first argument) into s0 (a register that things we call are not
    # allowed to fuck up)
    mv s0, a0
    # now that it's in s0, we can call `sbi_console_putchar` without it being
    # "lost"
.Lputstr_loop:
    # let ch = read_byte(pointer);
    lb a0, 0(s0)
    # if(ch == 0) break;
    beq a0, x0, .Lputstr_end_of_loop
    # sbi_console_putchar(ch);
    #mv a0, a0 # set up its first argument
    call sbi_console_putchar
    # pointer += 1;
    addi s0, s0, 1
    # (and the loop comes back around)
    j .Lputstr_loop
.Lputstr_end_of_loop:
    # restore s0 and the return address
    lw s0, 0(sp)
    lw ra, 4(sp)
    # restore the stack pointer
    addi sp, sp, 8
    # return!
    ret

# function putstring(pointer) {
#     do {
#         let ch = read_byte(pointer);
#         if(ch == 0) break;
#         sbi_console_putchar(ch);
#         pointer += 1;
#     } while(true);
# }














# doesn't start with ".L", is not an internal (local) label
# putstring:
#     mv t5, a0
#     mv t4, ra # save return address
# .Lputstr_loop:
#     # fucking intel notation!
#     lb t3, 0(t5)
#     beq t3, x0, .Lreturn
#     mv a0, t3
#     call sbi_console_putchar
#     addi t5, t5, 1
#     j .Lputstr_loop
#     # call THING is the same as:
#     #jal ra, THING
#     # ret is the same as:
#     #jr ra
# # ".L" = internal label
# .Lreturn:
#     mv ra, t4 # restore return address
    # ret

sbi_shutdown:
    # When making a Supervisor Binary Interface call...
    # a6: Function ID (always 0 for our purposes)
    li a6, 0
    # a7: Extension ID (8 = "shut the fuck down")
    li a7, 8
    # Call the Supervisor Binary Interface and--
    ecall

sbi_console_putchar:
    # a6: Function ID (always 0 for our purposes)
    li a6, 0
    # a7: Extension ID (1 = "put the fuck char")
    li a7, 1
    # Call the Supervisor Binary Interface
    ecall
    # and return!
    ret

sbi_console_getchar:
    # a6: Function ID (always 0 for our purposes)
    li a6, 0
    # a7: Extension ID (2 = "get the fuck char")
    li a7, 2
    # Call the Supervisor Binary Interface
    ecall
    # and return!
    ret
