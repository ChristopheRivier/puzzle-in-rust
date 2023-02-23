use std::cmp;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub trait AfficheDebug {
    fn to_string(&self) -> char;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Vide,
    Mur,
    Unknown,
    Base,
    Control,
}

impl AfficheDebug for Cell {
    fn to_string(&self) -> char {
        match self {
            Cell::Base => 'T',
            Cell::Control => 'C',
            Cell::Mur => '#',
            Cell::Unknown => '?',
            Cell::Vide => '.',
        }
    }
}

pub struct Game {
    kirk_row: i32,
    kirk_col: i32,
    carte: Vec<Vec<Cell>>,
    size_row: i32,
    size_col: i32,
    init_pos_r: i32,
    init_pos_c: i32,
    find_control: bool,
    control_present: bool,
}

impl Game {
    pub fn new(s: i32, c: i32) -> Game {
        Game {
            kirk_row: 0,
            kirk_col: 0,
            carte: Vec::new(),
            size_row: s,
            size_col: c,
            init_pos_c: -1,
            init_pos_r: -1,
            find_control: false,
            control_present: false,
        }
    }
    fn get_north(&self, r: i32, c: i32) -> (i32, i32) {
        if r - 1 < 0 {
            (-1, -1)
        } else {
            (r - 1, c)
        }
    }
    fn get_south(&self, r: i32, c: i32) -> (i32, i32) {
        if r + 1 >= self.size_row {
            (-1, -1)
        } else {
            (r + 1, c)
        }
    }
    fn get_eath(&self, r: i32, c: i32) -> (i32, i32) {
        if c - 1 < 0 {
            (-1, -1)
        } else {
            (r, c - 1)
        }
    }
    fn get_west(&self, r: i32, c: i32) -> (i32, i32) {
        if c + 1 >= self.size_col {
            (-1, -1)
        } else {
            (r, c + 1)
        }
    }
    pub fn add_line(&mut self, line: Vec<Cell>) {
        if line.iter().any(|c| c == &Cell::Control) {
            self.control_present = true;
        }
        self.carte.push(line);
    }
    pub fn kirk_position(&mut self, x: i32, y: i32) {
        self.kirk_row = x;
        self.kirk_col = y;
        if self.init_pos_c == -1 {
            self.init_pos_c = self.kirk_col;
            self.init_pos_r = self.kirk_row;
        }
    }

    fn alter_chemin(&self, chemin: &mut Vec<Vec<i32>>, r: i32, c: i32, valeur_courante: i32) {
        if r != -1 {
            if self.carte[r as usize][c as usize] != Cell::Mur
                && (chemin[r as usize][c as usize] == -1
                    || valeur_courante + 1 < chemin[r as usize][c as usize])
            {
                chemin[r as usize][c as usize] = valeur_courante + 1;
                self.find_next(chemin, r, c);
            }
        }
    }
    fn find_next(&self, chemin: &mut Vec<Vec<i32>>, r: i32, c: i32) {
        let valeur_courante = chemin[r as usize][c as usize];
        //eprintln!("valeur courante {} for {} {}", valeur_courante, r, c);

        let n = self.get_north(r, c);
        self.alter_chemin(chemin, n.0, n.1, valeur_courante);

        let s = self.get_south(r, c);
        self.alter_chemin(chemin, s.0, s.1, valeur_courante);

        let s = self.get_eath(r, c);
        self.alter_chemin(chemin, s.0, s.1, valeur_courante);

        let s = self.get_west(r, c);
        self.alter_chemin(chemin, s.0, s.1, valeur_courante);
    }

    fn find_control_ok(&mut self, r: i32, c: i32) {
        if self.carte[r as usize][c as usize] == Cell::Control {
            self.find_control = true;
        }
    }
    fn get_val_min(&mut self, r: i32, c: i32, carte: &mut Vec<Vec<i32>>) -> i32 {
        let n = self.get_north(r, c);
        let s = self.get_south(r, c);
        let e = self.get_eath(r, c);
        let w = self.get_west(r, c);

        let mut val_n = -1;
        if n.0 != -1 {
            val_n = carte[n.0 as usize][n.1 as usize];
        }
        let mut val_s = -1;
        if s.0 != -1 {
            val_s = carte[s.0 as usize][s.1 as usize];
        }
        let mut val_e = -1;
        if e.0 != -1 {
            val_e = carte[e.0 as usize][e.1 as usize];
        }
        let mut val_w = -1;
        if w.0 != -1 {
            val_w = carte[w.0 as usize][w.1 as usize];
        }
        let mut tmp: Vec<i32> = vec![val_e, val_n, val_s, val_w];
        tmp.retain(|&t| t >= 0);
        tmp.sort();
        if tmp.len() > 0 {
            tmp[0]
        } else {
            -1
        }
    }
    pub fn calcul_where_to_go(&mut self) -> String {
        let mut ret: String = String::from("");
        let mut chemin: Vec<Vec<i32>> =
            vec![vec![-1; self.size_col as usize]; self.size_row as usize];
        let mut cherche: Vec<Vec<i32>> =
            vec![vec![-2; self.size_col as usize]; self.size_row as usize];
        if !self.control_present {
            //calcul cherche
            for i in 0..self.size_row as usize {
                let mut ss: String = String::from("");
                for j in 0..self.size_col as usize {
                    cherche[i][j] = match self.carte[i][j] {
                        Cell::Mur => -1,
                        Cell::Unknown => 0,
                        _ => -2,
                    };
                }
            }
            // iter un certains nombre de fois
            for i in 0..self.size_row as usize {
                let mut ss: String = String::from("");
                for j in 0..self.size_col as usize {
                    if cherche[i][j] == -2 {
                        let mut val_min = self.get_val_min(
                            i.try_into().unwrap(),
                            j.try_into().unwrap(),
                            &mut cherche,
                        );
                        if val_min >= 0 {
                            cherche[i][j] = val_min + 1;
                        }
                    }
                }
            }
        }
        // calcul à partir du départ.
        for i in 0..self.size_row as usize {
            let mut ss: String = String::from("");
            for j in 0..self.size_col as usize {
                ss.push(self.carte[i][j].to_string());
            }
            //eprintln!("{}",ss);
        }
        // check à partir de T
        // init if cell == T
        if self.carte[self.init_pos_r as usize][self.init_pos_c as usize] == Cell::Base {
            chemin[self.init_pos_r as usize][self.init_pos_c as usize] = 0;
        }
        self.find_next(&mut chemin, self.init_pos_r, self.init_pos_c);
        // check max to go
        let n = self.get_north(self.kirk_row, self.kirk_col);
        let s = self.get_south(self.kirk_row, self.kirk_col);
        let e = self.get_eath(self.kirk_row, self.kirk_col);
        let w = self.get_west(self.kirk_row, self.kirk_col);

        let mut val_n = -1;
        if n.0 != -1 {
            val_n = chemin[n.0 as usize][n.1 as usize];
        }
        let mut val_s = -1;
        if s.0 != -1 {
            val_s = chemin[s.0 as usize][s.1 as usize];
        }
        let mut val_e = -1;
        if e.0 != -1 {
            val_e = chemin[e.0 as usize][e.1 as usize];
        }
        let mut val_w = -1;
        if w.0 != -1 {
            val_w = chemin[w.0 as usize][w.1 as usize];
        }
        eprintln!("{} {} {} {} ", val_n, val_s, val_e, val_w);
        if self.find_control {
            // back to T
            if val_n != -1
                && (val_n <= val_e || val_e == -1)
                && (val_n <= val_s || val_s == -1)
                && (val_n <= val_w || val_w == -1)
            {
                ret = String::from("UP");
            } else if val_s != -1
                && (val_s <= val_n || val_n == -1)
                && (val_s <= val_e || val_e == -1)
                && (val_s <= val_w || val_w == -1)
            {
                ret = String::from("DOWN");
            } else if val_e != -1
                && (val_e <= val_n || val_n == -1)
                && (val_e <= val_s || val_s == -1)
                && (val_e <= val_w || val_w == -1)
            {
                ret = String::from("LEFT");
            } else if val_w != -1
                && (val_w <= val_n || val_n == -1)
                && (val_w <= val_s || val_s == -1)
                && (val_w <= val_e || val_e == -1)
            {
                ret = String::from("RIGHT");
            }
        } else {
            // go to control
            if val_n != -1 && val_n >= val_e && val_n >= val_s && val_n >= val_w {
                ret = String::from("UP");
                self.find_control_ok(n.0, n.1);
            } else if val_s != -1 && val_s >= val_n && val_s >= val_e && val_s >= val_w {
                ret = String::from("DOWN");
                self.find_control_ok(s.0, s.1);
            } else if val_e != -1 && val_e >= val_n && val_e >= val_s && val_e >= val_w {
                ret = String::from("LEFT");
                self.find_control_ok(e.0, e.1);
            } else if val_w != -1 && val_w >= val_n && val_w >= val_s && val_w >= val_e {
                ret = String::from("RIGHT");
                self.find_control_ok(w.0, w.1);
            }
        }

        for i in 0..self.size_row as usize {
            for j in 0..self.size_col as usize {
                //eprint!("{} ", chemin[i][j]);
            }
            //eprintln!("");
        }

        ret
    }
    fn clear_tab(&mut self) {
        self.carte.clear();
    }
}
fn init_ligne_cell(line: &String) -> Vec<Cell> {
    let mut ret: Vec<Cell> = Vec::new();
    for c in line.chars() {
        ret.push(match c {
            'C' => Cell::Control,
            '.' => Cell::Vide,
            '#' => Cell::Mur,
            'T' => Cell::Base,
            '?' => Cell::Unknown,
            _ => todo!(),
        })
    }
    ret
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i32); // number of rows.
    let c = parse_input!(inputs[1], i32); // number of columns.
    let a = parse_input!(inputs[2], i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.
    let mut game = Game::new(r, c);
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let kr = parse_input!(inputs[0], i32); // row where Rick is located.
        let kc = parse_input!(inputs[1], i32); // column where Rick is located.
        game.kirk_position(kr, kc);
        game.clear_tab();
        for i in 0..r as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim().to_string(); // C of the characters in '#.TC?' (i.e. one line of the ASCII maze).
            let ll = init_ligne_cell(&row);
            game.add_line(ll);
            eprintln!("{}", row)
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", game.calcul_where_to_go()); // Rick's next move (UP DOWN LEFT or RIGHT).
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_line_tab() {
        let line = ".TC?#";
        let a = [
            Cell::Vide,
            Cell::Base,
            Cell::Control,
            Cell::Unknown,
            Cell::Mur,
        ];
        let toto = init_ligne_cell(&line.to_string());
        for i in 0..a.len() as usize {
            assert_eq!(a[i], toto[i]);
        }
    }
    #[test]
    fn test_initialisation() {
        let mut game: Game = Game::new(3, 3);
        game.kirk_position(1, 1);
        game.add_line(init_ligne_cell(&String::from("###")));
        game.add_line(init_ligne_cell(&String::from("#T.")));
        game.add_line(init_ligne_cell(&String::from("##C")));
        assert_eq!(game.calcul_where_to_go(), "RIGHT".to_string());
        game.kirk_position(1, 2);
        game.add_line(init_ligne_cell(&String::from("###")));
        game.add_line(init_ligne_cell(&String::from("#T.")));
        game.add_line(init_ligne_cell(&String::from("##C")));
        assert_eq!(game.calcul_where_to_go(), "DOWN".to_string());
    }
    #[test]
    fn test_map() {
        let mut game: Game = Game::new(3, 4);
        game.kirk_position(0, 0);
        game.add_line(init_ligne_cell(&String::from("T..?")));
        game.add_line(init_ligne_cell(&String::from("##??")));
        game.add_line(init_ligne_cell(&String::from("##??")));
        assert_eq!(game.calcul_where_to_go(), "RIGHT".to_string());
        game.kirk_position(0, 1);
        game.add_line(init_ligne_cell(&String::from("T..?")));
        game.add_line(init_ligne_cell(&String::from("##??")));
        game.add_line(init_ligne_cell(&String::from("##??")));
        assert_eq!(game.calcul_where_to_go(), "RIGHT".to_string());
        game.kirk_position(0, 2);
        game.add_line(init_ligne_cell(&String::from("T...")));
        game.add_line(init_ligne_cell(&String::from("##..")));
        game.add_line(init_ligne_cell(&String::from("##??")));
        assert_eq!(game.calcul_where_to_go(), "DOWN".to_string());
        game.kirk_position(1, 2);
        game.add_line(init_ligne_cell(&String::from("T...")));
        game.add_line(init_ligne_cell(&String::from("##..")));
        game.add_line(init_ligne_cell(&String::from("##.C")));
        assert_eq!(game.calcul_where_to_go(), "DOWN".to_string());
    }
    #[test]
    fn test_reelle() {
        let mut game: Game = Game::new(15, 30);
        game.kirk_position(6, 5);
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???#####??????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???#####??????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???##T..??????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???#####??????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???#####??????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        assert_eq!(game.calcul_where_to_go(), "RIGHT".to_string());
        game.kirk_position(6, 11);
        game.clear_tab();
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???##T......C#????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        assert_eq!(game.calcul_where_to_go(), "RIGHT".to_string());
        game.kirk_position(6, 12);
        game.clear_tab();
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???##T......C#????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        assert_eq!(game.calcul_where_to_go(), "LEFT".to_string());
        game.kirk_position(6, 11);
        game.clear_tab();
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???##T......C#????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "???############???????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        game.add_line(init_ligne_cell(&String::from(
            "??????????????????????????????",
        )));
        assert_eq!(game.calcul_where_to_go(), "LEFT".to_string());
    }
}
