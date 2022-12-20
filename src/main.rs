use std::fmt; ///formatter
use std::fmt::{Display, Formatter, Error};

enum VertDir{ ///for up and down moving
    Up, 
    Down
}

enum HorizDir{ ///left and right movement
    Left, 
    Right
}


struct Ball{ /// holding and defining value of enum
    x : i32, 
    y : i32,
    vert_dir : VertDir, 
    horiz_dir : HorizDir
}


struct Frame{ ///dimensions of the box
    width : i32, 
    height : i32, 
}


struct Game{ ///game holds the frame and the ball needed to make the ball move in terminal
    frame : Frame, 
    ball : Ball
}

impl Game {
    fn new()->Game{
     
     Game{
            frame : Frame{
            width : 63,
            height : 31 
            },

            ball: Ball{
                x : 44,
                y : 21, 
                vert_dir : VertDir::Down,
                horiz_dir : HorizDir::Right 
            }
     }
    }

    fn step(&mut self){
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }

}


impl Ball{
    fn bounce(&mut self, frame: &Frame){
        if self.x <=0 {
            self.horiz_dir = HorizDir::Right;
        }
        else if frame.width<=self.x{
            self.horiz_dir = HorizDir::Left;
        }
        else if self.y<=0{
            self.vert_dir = VertDir::Down;
        }
        else if frame.height<=self.y{
            self.vert_dir = VertDir::Up;
        }

        else{
               
        }

    }

    fn mv(&mut self){
        match self.horiz_dir{
            HorizDir::Left => self.x-=1,
            HorizDir::Right => self.x+=1
        }
        match self.vert_dir{
            VertDir::Up => self.y-=1,
            VertDir::Down => self.y+=1
        }
    }
}


impl Display for Game{ ///dimensions and how the ball moves in display
    fn fmt(&self, fmt:&mut Formatter)->Result<(), Error>{
        write!(fmt, "x");
        for _ in 0..64{ write!(fmt, "-"); }
        for y in 0..32{
            for x in 0..64{
                if self.ball.x == x as i32 && self.ball.y == y as i32{
                    write!(fmt, "O");
                }
                if x == 0 {write!(fmt, "|");} else if x!=0 && y!=31 {write!(fmt, " ");} else {write!(fmt,"-");}
            }
            write!(fmt, "|\n");
        }

        write!(fmt, "\n")
       // write!(fmt, "{} {}", self.ball.x, self.ball.y)
    }
}


fn main() {
     let mut new_game  = Game::new();
     let sleep_time = std::time::Duration::from_millis(33); ///represent a span of time, typically used for system timeouts
     loop {
            println!("{}", new_game); 
            new_game.step();
            std::thread::sleep(sleep_time); ///resprents a sleep time 
            println!("{} {} ", new_game.ball.x, new_game.ball.y);
     }


}
