#[derive(Clone, Debug)]
pub struct Pawn{
    position: (usize, usize),
    player: String,
    class: String,
    color: String,
    alive: bool,
}

impl Pawn{
    pub fn new(position: (usize, usize), player: String, class: String, color: String, alive: bool) -> Self{
        Pawn{position, player, class, color, alive}
    }

    pub fn set_position(&mut self, new_position: (usize, usize)){
        self.position = new_position
    }

    pub fn get_position(&self) -> &(usize, usize){
        &self.position
    }
    
    pub fn get_class(&self) -> &str{
        self.class.as_str()
    }

    pub fn set_alive(&mut self, alive: bool){
        self.alive = alive
    }

    pub fn get_alive(&self) -> bool{
        self.alive
    }

    pub fn get_color(&self) -> &str{
        self.color.as_str()
    }
}