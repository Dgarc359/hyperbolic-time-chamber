```js
function putstring(pointer) {
    do {
        let ch = read_byte(pointer);
        if(ch == 0) break;
        sbi_console_putchar(ch);
        pointer += 1;
    } while(true);
}
```

```js
// putint(12345)
//                                v
// array_of_digits = [5, 4, 3, 2, 1]

// ONLY CALL THIS FUNCTION ON NUMBERS LESS THAN OR EQUAL TO 4294967295
function putint(n) {
    let array_of_digits = reserve_some_space(10);
    let next_digit_to_output = array_of_digits;
    do {
        let temp = character_code_of('0') + (n % 10);
        next_digit_to_output[0] = temp;
        next_digit_to_output += 1;
        n = n / 10; // pretend it's not JavaScript and this is integer divison
    } while(n > 0);
    let ch;
    do {
        next_digit_to_output -= 1;
        ch = next_digit_to_output[0];
        sbi_console_putchar(ch);
    } while (next_digit_to_output > array_of_digits)
}
function putint(n) {
    let array_of_digits = reserve_some_space(10);
    let number_of_digits = 0;
    let next_digit_to_output = array_of_digits;
    do {
        next_digit_to_output[0] = character_code_of('0') + (n % 10);
        next_digit_to_output += 1;
        number_of_digits += 1;
        n = n / 10; // pretend it's not JavaScript and this is integer divison
    } while(n > 0);
    let ch;
    let i = number_of_digits - 1;
    do {
        ch = array_of_digits[i];
        sbi_console_putchar(ch);
        i--;
    } while (i >= 0)
}
```

```js
putstring("Welcome to Fight!");
let uhp = 50;
let mhp = 100;
let upot = 3;
while(uhp > 0 && mhp > 0) {
    putstring("You have: ");
    putint(uhp);
    putstring(" / 50 HP. Potions: ");
    putint(upot);
    putstring("\nEnemy has: ");
    putint(mhp);
    putstring(" / 100 HP\nWhat will you do? (a)ttack, or (p)otion? ");
    let wat;
    do {
        wat = sbi_console_getchar();
    } while(wat < 0);
    sbi_console_putchar(wat);
    sbi_console_putchar('\n');
    if(wat == 'a') {
        putstring("You attack, dealing 10 damage!\n");
        mhp -= 10;
    } else if(wat == 'p') {
        if(upot > 0) {
            putstring("You drink a potion, healing 30 hitpoints.\n");
            uhp += 30;
            if(uhp > 50) {
                putstring("Some was wasted!\n");
                uhp = 50;
            }
            upot -= 1;
        } else {
            putstring("You are out of potions. nerd.\n");
            continue;
        }
    } else {
        putstring("You must type the letter a or the letter p.\n");
        continue;
    }
    putstring("The enemy attacks, dealing 10 damage!\n");
    uhp -= 10;
}
if(uhp <= 0) {
    putstring("You died.\n");
    if(mhp <= 0) {
        putstring("The fact that your enemy died too is small consolation for you.\n");
    }
} else {
    putstring("You didn't die!\n");
}
sbi_shutdown();
```