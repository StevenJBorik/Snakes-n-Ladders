#[macro_use]
extern crate text_io; 
extern crate termion;
use rand::Rng; 
use std::collections::HashMap;
use terminon::color; 

// Snl class [Parameters]:
    // Number of players
    // Player state information
pub struct Snl {
    numPlayers: NumPlayers,
    players: Players<'static>, 
}

// Create new game object
impl Snl {
    pub fn new() -> Snl {
        Snl {
            numPlayers: NumPlayers::new(),
            players: Players::new(),
        }
    }
    // draw initial board state
    pub fn gameStart(&mut self) {
        let numPlayers = self.numPlayers.getPlayers(); 
        let arrPlayers = self.players.updatePlayerIno(numPlayers); 
        self.players.gamePlay(arrPlayers, numPlayers); 
    }
}

// # of players class
pub struct NumPlayers {
    numPlayers: usize,
}

// Create new player object, set to 0
impl NumPlayers {
    pub fn new() -> NumPlayers {
        Numplayers { numPlayers: 0 }
    }

    pub fn getPlayers(&mut self) -> usize {
        println!("{:?}", "Plz enter number of climbers. 2, 3, or 4?"); 
        // select # of players and return value
        let mut numPlayers: usize; 
        while numPlayers > 4 || numPlayers < 2 {
            println("{:?}", "Nice try, of course I added this validation! Now, you feelin lucky, punk? 2, 3, or 4?");
            scan!("{}", numPlayers); 
        }
        self.numPlayers = numPlayers;
        numPlayers
    }
}

//    Struct for each player in the game
//    position: player position in the board
//    avatar: each player is assigned a unique avatar
//    avatar_overlap: if players land on same block in the board
//    avatar_display: displays player avatar

#[derive(Copy, Clone)]
struct Players<'a> {
    position: usize,
    avatar: &'a str,
    avatar_overlap: &'a str,
    avatar_display: &'a str,
}

impl<'a> players<'a> {
    // Player Constructor
    pub fn new() -> Players<'static>{
        let arr_players = Players {
            avatar: "|*A*       ",
            avatar_overlap: "|*A*#B#    ",
            avatar_display: "*A*",
        }; 
        arr_players
    }

    // Initializes player's board position
    pub fn initializePlayerPos(&mut self, numplayers: usize) -> Vec<players<'static>>{
            // use vec of type players to push all player related data
            let mut arr_players = vec![]; 
            match numplayers {
                2 => {
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|*A*       ",
                        avatar_overlap: "|*A*#B#    ",
                        avatar_display: "*A*",
                    });
                    println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|#B#       ",
                        avatar_overlap: "|*A*#B#    ",
                        avatar_display: "#B#",
                    });
                    println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
                }
                3 => {
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|*A*       ",
                        avatar_overlap: "|*A*#B#$C$ ",
                        avatar_display: "*A*",
                    });
                    println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|#B#       ",
                        avatar_overlap: "|*A*#B#$C$ ",
                        avatar_display: "#B#",
                    });
                    println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|$C$       ",
                        avatar_overlap: "|*A*#B#$C$ ",
                        avatar_display: "$C$",
                    });
                    println!("Player C's avatar is {:?}", arr_players[2].avatar_display);
                }
                4 => {
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|*A*       ",
                        avatar_overlap: "|*A#B$C%D  ",
                        avatar_display: "*A*",
                    });
                    println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|#B#       ",
                        avatar_overlap: "|*A#B$C%D  ",
                        avatar_display: "#B#",
                    });
                    println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|$C$       ",
                        avatar_overlap: "|*A#B$C%D  ",
                        avatar_display: "$C$",
                    });
                    println!("Player C's avatar is {:?}", arr_players[2].avatar_display);
                    arr_players.push(Players {
                        position: 1,
                        avatar: "|%D%       ",
                        avatar_overlap: "|*A#B$C%D  ",
                        avatar_display: "%D%",
                    });
                    println!("Player D's avatar is {:?}", arr_players[3].avatar_display);
                }
                _ => (),

            }
            arr_players
        }
    }

    // Hash Function
    pub fn snakesAndLaddersHash(&mut self, position: usize) -> usize {
        let mut ladders = Hashmap::new(); 
        let mut new_position = position;
        
        ladders.insert(02, 22);
        ladders.insert(05, 11);
        ladders.insert(09, 17);
        ladders.insert(18, 32);
        ladders.insert(29, 36);
        ladders.insert(38, 57);
        ladders.insert(47, 68);
        ladders.insert(53, 64);
        ladders.insert(69, 76);
        ladders.insert(74, 92);
        ladders.insert(83, 91);
        ladders.insert(89, 95);

        if let Some(x) = ladders.get(&position) {
            println!("YAY! You rolled & reached a Ladder!!");
            new_position = *x;
            println!(
                "You reach {:?} and climb your way up to {:?}",
                position, new_position
            );
        }

        //Defining a hash for the snakes
        let mut snakes = HashMap::new();
        snakes.insert(13, 1);
        snakes.insert(25, 4);
        snakes.insert(35, 16);
        snakes.insert(44, 22);
        snakes.insert(66, 51);
        snakes.insert(77, 45);
        snakes.insert(86, 61);
        snakes.insert(94, 82);
        snakes.insert(98, 78);

            //check if a player gets a snake
        if let Some(x) = snakes.get(&position) {
            println!("Oh NO!!!! You got bit by a Snake!!***");
            new_position = *x;
            println!(
                "You reach {:?} to slide all the way back to {:?}",
                position, new_position
            );
        }
    new_position
    }

    pub fn gamePlay(&mut self, mut arr_players: Vec<Players<'static>>, numPlayers: usize) {
        // Define board layout

        let border = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! S  ~SNAKES & LADDERS-> R !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
        let input = vec![
            vec![
                "|Begin     ",
                "|02->[22]  ",
                "|03        ",
                "|04        ",
                "|05->[11]  ",
                "|06        ",
                "|07        ",
                "|08        ",
                "|09->[17]  ",
                "|10         |",
            ],
            vec![
                "|11        ",
                "|12        ",
                "|13~{Begin}",
                "|14        ",
                "|15        ",
                "|16        ",
                "|17        ",
                "|18->[32]  ",
                "|19        ",
                "|20         |",
            ],
            vec![
                "|21        ",
                "|22        ",
                "|23        ",
                "|24        ",
                "|25~{04}   ",
                "|26        ",
                "|27        ",
                "|28        ",
                "|29->[36]  ",
                "|30         |",
            ],
            vec![
                "|31        ",
                "|32        ",
                "|33        ",
                "|34        ",
                "|35~{16}   ",
                "|36        ",
                "|37        ",
                "|38->[57]  ",
                "|39        ",
                "|40         |",
            ],
            vec![
                "|41        ",
                "|42        ",
                "|43        ",
                "|44~{22}   ",
                "|45        ",
                "|46        ",
                "|47->[68]  ",
                "|48        ",
                "|49        ",
                "|50         |",
            ],
            vec![
                "|51        ",
                "|52        ",
                "|53->[64]  ",
                "|54        ",
                "|55        ",
                "|56        ",
                "|57        ",
                "|58        ",
                "|59        ",
                "|60         |",
            ],
            vec![
                "|61        ",
                "|62        ",
                "|63        ",
                "|64        ",
                "|65        ",
                "|66~{51}   ",
                "|67        ",
                "|68        ",
                "|69->[76]  ",
                "|70         |",
            ],
            vec![
                "|71        ",
                "|72        ",
                "|73        ",
                "|74->[92]  ",
                "|75        ",
                "|76        ",
                "|77~{45}   ",
                "|78        ",
                "|79        ",
                "|80         |",
            ],
            vec![
                "|81        ",
                "|82        ",
                "|83->[91]  ",
                "|84        ",
                "|85        ",
                "|86~{61}   ",
                "|87        ",
                "|88        ",
                "|89->[95]  ",
                "|90         |",
            ],
            vec![
                "|91        ",
                "|92        ",
                "|93        ",
                "|94->{82}  ",
                "|95        ",
                "|96        ",
                "|97        ",
                "|98~{78}   ",
                "|99        ",
                "|End        |",
            ],
        ];

        let mut row_num: [usize; 4] = [0; 4];
        let mut column: [usize; 4] = [0; 4];

        //Draw initial layout
        for k in 0..num_players {
            row_num[k] = (arr_players[k].position - 1) / 10;
            column[k] = (arr_players[k].position - 1) % 10;
        }
         //Display the board
         println!();
         println!("**** {:?} Players have entered the pit - Chaos is a ladder [or a snake]. You either climb or get bit! Roll & enjoy the chaos...****", num_players);
         println!("*************Snakes & Ladders initial board layout: ***************");
         println!("{:?}", border);
         for (i, row) in input.iter().enumerate() {
             for (j, mut col) in row.iter().enumerate() {
                 for k in 0..num_players {
                     if i == row_num[k] && j == column[k] {
                         col = &arr_players[k].avatar_overlap;
                     }
                 }
                 print!("{}", col);
             }
             println!()
         }
         println!("{:?}", border);
 

    }



    
