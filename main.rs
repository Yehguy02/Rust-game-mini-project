use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use rand::Rng;
use std::io::stdout;
use std::io;
use std::time::Duration;
use std::thread;

fn main() {
    let mut is_start = false;
    let mut is_player_dead = false;
    let mut is_enemy_dead = false;
    let mut p_monster_choice = 0;
    let mut p_action_choice = 0; // 1 attack, 2 guard, 3 buff, 4 counter, 5 change, 6 stat
    let mut e_monster_choice = 0;
    let mut e_action_choice = 0;

    let mut player: Vec<Monster> = vec![
        Monster { hp: 80.0, max_hp: 80.0, atk: 7.0, spd: 6.0, typing: Types::Fire, action: Moves::None, is_alive: true },
        Monster { hp: 100.0, max_hp: 100.0, atk: 5.0, spd: 4.0, typing: Types::Water, action: Moves::None, is_alive: true },
        Monster { hp: 120.0, max_hp: 120.0, atk: 4.0, spd: 3.0, typing: Types::Grass, action: Moves::None, is_alive: true },
        Monster { hp: 90.0, max_hp: 90.0, atk: 6.0, spd: 5.0, typing: Types::Normal, action: Moves::None, is_alive: true },
    ];
    let mut enemy: Vec<Monster> = vec![
        Monster { hp: 80.0, max_hp: 80.0, atk: 7.0, spd: 6.0, typing: Types::Fire, action: Moves::None, is_alive: true },
        Monster { hp: 100.0, max_hp: 100.0, atk: 5.0, spd: 4.0, typing: Types::Water, action: Moves::None, is_alive: true },
        Monster { hp: 120.0, max_hp: 120.0, atk: 4.0, spd: 3.0, typing: Types::Grass, action: Moves::None, is_alive: true },
        Monster { hp: 90.0, max_hp: 90.0, atk: 6.0, spd: 5.0, typing: Types::Normal, action: Moves::None, is_alive: true },
    ];

    loop {
        if !is_start { // starting menu, choosing monster to start, run only once
            is_start = true;
            p_monster_choice = start_menu();
            e_monster_choice = random(4);
            wait(1);
            // continue
        }
        if !player[0].is_alive && !player[1].is_alive && !player[2].is_alive && !player[3].is_alive { // check if all are dead, if yes then game should end
            is_player_dead = true
        }
        if !enemy[0].is_alive && !enemy[1].is_alive && !enemy[2].is_alive && !enemy[3].is_alive {
            is_enemy_dead = true
        }
        
        if !is_player_dead && !is_enemy_dead && is_start { // main game happens here 
            e_action_choice = random(15) + 1;

            if !player[p_monster_choice].is_alive { // if player's monster is dead, choose new one
                // clear_screen();
                println!("\n* - * - * - * - * - * - * - * - * - * - * - *");
                println!("Your monster had died!\nChoose new monster to battle!");
                for i in 0..player.len() {
                    println!("{}. {:?} type monster (is alive: {})", i + 1, player[i].typing, player[i].is_alive);
                }
                loop {
                    p_monster_choice = get_int_input() - 1;
                    if player[p_monster_choice].is_alive && p_monster_choice <= 4 && p_monster_choice > 0 {
                        break;
                    } else {
                        println!("Choose an alive monster");
                    }
                }
            }
            wait(1);
            match e_action_choice {
                1..=8 => enemy[e_monster_choice].set_action(Moves::Attack),
                9 => enemy[e_monster_choice].set_action(Moves::Guard),
                10 => enemy[e_monster_choice].set_action(Moves::Buff),
                11..=14 => enemy[e_monster_choice].set_action(Moves::Counter),
                15 => enemy[e_monster_choice].set_action(Moves::None),
                _ => {
                    println!("Enemy does nothing la! {}", e_action_choice);
                }
            }

            loop {
                p_action_choice = choosing_action(&mut player[p_monster_choice as usize], &mut enemy[e_monster_choice as usize]);
                match p_action_choice {
                    1 => {
                        player[p_monster_choice].set_action(Moves::Attack);
                        break
                    },
                    2 => {
                        player[p_monster_choice].set_action(Moves::Guard);
                        break
                    },
                    3 => {
                        player[p_monster_choice].set_action(Moves::Buff);
                        break
                    },
                    4 => {
                        player[p_monster_choice].set_action(Moves::Counter);
                        break
                    },
                    5 => {
                        loop {
                            p_monster_choice = choosing_monster(&player);
                            p_action_choice = 0;
                            player[p_monster_choice].set_action(Moves::None);
                            if player[p_monster_choice].hp > 0.0 { break }
                        }
                        break
                    },
                    6 => {
                        for i in player.iter() {
                            i.print_stat();
                        }
                        loop {
                            println!("Go back? (y / n)");
                            let mut input: String = String::new();
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let yes_or_no: String = input.trim().to_string();
                            if yes_or_no.to_uppercase().eq("Y") { break }
                        }
                    },
                    _ => println!("Please enter a valid choice!"),
                }
            }
            wait(1);

            if enemy[e_monster_choice].spd > player[p_monster_choice].spd {
                match enemy[e_monster_choice].action {
                    Moves::Attack => {
                        println!("The enemy's monster attack your monster!");
                        enemy[e_monster_choice].attack(&mut player[p_monster_choice]);
                        println!("Your monster's hp: {:.1}\nEnemy's monter's hp: {:.1}", player[p_monster_choice].hp, enemy[e_monster_choice].hp);
                    },
                    Moves::Buff => {
                        println!("The enemy's monster buff itself!");
                        enemy[e_monster_choice].buff();
                    },
                    Moves::Counter => println!("Becareful, the enemy's monster prepare a counter!"),
                    Moves::Guard => println!("The enemy's monster use guard!"),
                    Moves::None => {
                        println!("Enemy has change monster!");
                        wait(1);
                        loop {
                            e_monster_choice = random(4);
                            if enemy[e_monster_choice].hp > 0.0 { break }
                        }
                        println!("Enemy's monster has changed to {:?} type monster!", enemy[e_monster_choice].typing);
                    },
                }
                wait(1);

                if player[p_monster_choice].hp > 0.0 {
                    match p_action_choice {
                        1 => {
                            println!("You have attacked enemy's monster!");
                            player[p_monster_choice].attack(&mut enemy[e_monster_choice]);
                            println!("Your monster's hp: {:.1}\nEnemy's monter's hp: {:.1}", player[p_monster_choice].hp, enemy[e_monster_choice].hp);
                        },
                        2 => { 
                            println!("Your monster uses guard!");
                        },
                        3 => {
                            println!("Your monster's buff it's stat!");
                            player[p_monster_choice].buff();
                        },
                        4 => {
                            println!("Your monster's uses counter!");
                        },
                        5 => {
                            println!("Which monster will you choose to battle next?");
                            loop {
                                p_monster_choice = choosing_monster(&player);
                                player[p_monster_choice].set_action(Moves::None);
                                if player[p_monster_choice].hp > 0.0 { break }
                            }
                        },
                        _ => {
                            println!("Your monster is ready to fight!");
                        }
                    }
                }
            } else {
                match p_action_choice {
                    1 => {
                        println!("You have attacked enemy's monster!");
                        player[p_monster_choice].attack(&mut enemy[e_monster_choice]);
                        println!("Your monster's hp: {:.1}\nEnemy's monter's hp: {:.1}", player[p_monster_choice].hp, enemy[e_monster_choice].hp);
                    },
                    2 => { 
                        println!("Your monster have guarded enemy's attack!");
                    },
                    3 => {
                        println!("Your monster's buff it's stat!");
                        player[p_monster_choice].buff();
                    },
                    4 => {
                        println!("Your monster's will counter enemy's attack!");
                    },
                    5 => {
                        println!("Which monster will you choose to battle next?");
                        loop {
                            p_monster_choice = choosing_monster(&player);
                            player[p_monster_choice].set_action(Moves::None);
                            if player[p_monster_choice].hp > 0.0 { break }
                        }
                    },
                    _ => {
                        println!("Your monster is ready to fight!")
                    }
                }
                wait(1);

                if enemy[e_monster_choice].hp > 0.0 {
                    match enemy[e_monster_choice].action {
                        Moves::Attack => {
                            println!("The enemy's monster attack your monster!");
                            enemy[e_monster_choice].attack(&mut player[p_monster_choice]);
                            println!("Your monster's hp: {:.1}\nEnemy's monter's hp: {:.1}", player[p_monster_choice].hp, enemy[e_monster_choice].hp);
                        },
                        Moves::Buff => {
                            println!("The enemy's monster buff itself!");
                            enemy[e_monster_choice].buff();
                        },
                        Moves::Counter => println!("Becareful, the enemy's monster prepare a counter!"),
                        Moves::Guard => println!("The enemy's monster use guard!"),
                        Moves::None => {
                            println!("Enemy has change monster!");
                            wait(1);
                            loop {
                                e_monster_choice = random(4);
                                if enemy[e_monster_choice].hp > 0.0 { break }
                            }
                            println!("Enemy's monster has changed to {:?} type monster!", enemy[e_monster_choice].typing);
                        },
                    }
                } else {
                    if enemy[0].is_alive || enemy[1].is_alive || enemy[2].is_alive || enemy[3].is_alive {
                        println!("Enemy has change monster!");
                        wait(1);
                        loop {
                            e_monster_choice = random(4);
                            if enemy[e_monster_choice].hp > 0.0 { break }
                        }
                        println!("Enemy's monster has changed to {:?} type monster!", enemy[e_monster_choice].typing);
                    }
                }
            }
        } else {
            if is_player_dead {
                println!("\nToo bad, you ran out of monster to fight!\n\nYou lose\n");
                break
            } else if is_enemy_dead {
                println!("\nCongratulations! You have defeated the enemy!\n\nYou win!\n");
                break
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Types {
    Fire,
    Water,
    Grass,
    Normal,
}

impl Types {
    fn compare_type(&self, enemy: Monster) -> f64 {
        match self {
            Types::Fire => {
                match enemy.typing {
                    Types::Water => 0.5,
                    Types::Grass => 1.5,
                    Types::Fire => 1.0,
                    Types::Normal => 1.2,
                }
            },
            Types::Water => {
                match enemy.typing {
                    Types::Water => 1.0,
                    Types::Grass => 0.5,
                    Types::Fire => 1.5,
                    Types::Normal => 1.2,
                }
            },
            Types::Grass => {
                match enemy.typing {
                    Types::Water => 1.5,
                    Types::Grass => 1.0,
                    Types::Fire => 0.5,
                    Types::Normal => 1.2,
                }
            },
            Types::Normal => {
                match enemy.typing {
                    Types::Water => 0.8,
                    Types::Grass => 0.8,
                    Types::Fire => 0.8,
                    Types::Normal => 1.0,
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Moves {
    Attack,
    Guard,
    Buff,
    Counter,
    None,
}

impl Moves {
    fn compare_action(&self, enemy: Monster) -> f64 {
        match enemy.action {
            Moves::Attack => 1.0,
            Moves::Buff => 1.0,
            Moves::Guard => 0.5,
            Moves::Counter => -1.0,
            Moves::None => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Monster {
    hp: f64,
    max_hp: f64,
    atk: f64,
    spd: f64,
    typing: Types,
    action: Moves,
    is_alive: bool,
}

impl Monster {
    fn set_action(&mut self, action: Moves) {
        self.action = action
    }
    fn attack(&mut self, enemy: &mut Monster) {
        let damage_dealt = self.atk * self.typing.compare_type(*enemy) * self.action.compare_action(*enemy);
        if damage_dealt < 0.0 { // < 0 is counter
            if self.hp + damage_dealt <= 0.0 {
                self.hp = 0.0;
                self.is_alive = false;
            } else {
                self.hp += damage_dealt;
            }
        } else {
            if enemy.hp - damage_dealt <= 0.0 {
                enemy.hp = 0.0;
                enemy.is_alive = false;
            } else {
                enemy.hp -= damage_dealt;
            }
        }
    }
    fn buff(&mut self) {
        self.atk += 5.0;
        self.spd += 1.0;
    }
    fn print_stat(&self) {
        println!("\n{:?} Type Monster's Stats", self.typing);
        println!("HP: {:.1}/{}", self.hp, self.max_hp);
        println!("Attack: {}, Speed: {}", self.atk, self.spd);
    }
}

fn random(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let a: f64 = rng.gen();
    (a * (max as f64)) as usize
}

fn clear_screen() {
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();
}

fn get_int_input() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Enter number"),
        }
    }
}

fn wait(sec: usize) {
    thread::sleep(Duration::from_secs(sec as u64));
}

fn start_menu() -> usize {
    loop {
        clear_screen();
        println!("\n* - * - * - * - * - * - * - * - * - * - * - *");
        println!("Welcome to Monster Battle!");
        println!("Choose one of your monster to begin battle!\n1. Fire monster\n2. Water monster\n3. Grass monster\n4. Normal monster\n");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num > 0 && num < 5 { break num - 1 }
            },
            Err(_) => println!("Enter number"),
        }
    }
}

fn choosing_action(player: &mut Monster, enemy: &mut Monster) -> usize {
    loop {
        clear_screen();
        println!("\n* - * - * - * - * - * - * - * - * - * - * - *");
        println!("Your monster: {:?} type monster.\n  Remaining hp: {:.1}/{}", player.typing, player.hp, player.max_hp);
        println!("Enemy's monster: {:?} type monster.\n  Remaining hp: {:.1}/{}", enemy.typing, enemy.hp, enemy.max_hp);
        println!("\nWhat will you do?\n1. Attack\n2. Guard\n3. Buff\n4. Counter\n5. Change monster\n6. Check monster's stat\n");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let temp = match input.trim().to_string().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if temp > 0 && temp <= 6 {
            break
            temp
        }
    }
}

fn choosing_monster(player: &Vec<Monster>) -> usize {
    loop {
        clear_screen();
        println!("\n* - * - * - * - * - * - * - * - * - * - * - *");
        println!("\nWhich monster will you change to\n");
        for i in 0..4 {
            print!("{}. ", i + 1);
            player[i].print_stat();
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let temp = match input.trim().to_string().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if temp > 0 && temp <= 4 {
            break
            temp - 1
        }
    }
}
