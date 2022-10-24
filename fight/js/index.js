const prompt = require('prompt-sync')();


const main = () => {
    const me = new Person(50, 3);
    const enemy = new Person(100, 0);

    while(me.hp !== 0 && enemy.hp !== 0) {
        console.log(`You have ${me.potions} potions and ${me.hp} hp`);
        console.log("What would you like to do?");
        const choice = prompt("> ");
        
         
        if(choice === 'attack') {
            enemy.hp = enemy.hp - 10;
            console.log(`Enemy hp is: ${enemy.hp}`);
        } else if (choice === 'potion') {
            if(me.potions === 0) {
                console.log("out of potions!");
                continue;
            }
            if(20 <= me.hp && me.hp <= 50) {
                me.hp = 50;
            } else {
                me.hp += 30;
            }
            me.potions -= 1;
        } else if (choice === 'quit') {
            console.log("quitting");
            return;
        } else {
            continue;
        }

        console.log("enemy is attacking!");
        me.hp -= 10;
    }

    console.log(`You finished with hp: ${me.hp}, the enemy had: ${enemy.hp}`);
    if(me.hp == 0 && enemy.hp === 0) {
        console.log("You both fought valiantly to the death, neither succeeded");
    } else if (me.hp === 0) {
        console.log("You were not able to best your enemy this time...");
    } else {
        console.log("The enemy gremlin was defeated");
    }
}

class Person {
    // hp;
    // potion;
    constructor(hp, potions) {
        this.hp = hp;
        this.potions = potions;
    }
}

main();