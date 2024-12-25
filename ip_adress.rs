// Cuando definimos un enum este puede aceptar valores asociados al valor.
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

// Vamos a definir mensajes y vamos a implementar un metodo que imprima los valores del mensaje
#[derive(Debug)]
enum Message { 
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn debug_call (&self){
	println!("{:?}", self)
    }
    
    fn call(&self){
	match self {
	    Message::Quit => println!("The message is Quit"),
	    Message::Move {x, y}=> println!("Move with values x->{}, y->{}", x, y),
	    Message::Write(text) => println!("{}", text),
	    Message::ChangeColor(r,g,b) => println!("Color: {} {} {}", r, g, b),
	}

    }
}


fn main(){ 
    // Asi se genera un enum con el valor que le asignamos. 
    // Tambien podriamos asignarle un structo.
    // let home = IpAddr::V4(127,0,0,1);
    // let loopback =  IpAddr::V6(String::from("::1");

    let m1 = Message::Quit;
    let m2 = Message::Move{x: 12, y: 34};
    //let m3 = Message::Write(String::from("Im a Write Message"));
    //let m4 = Message::ChangeColor(214,235,124);


    match m2{
	Message::Quit => std::process::exit(0),
	other => other.call(),
    }

    let config_max = Some(3u8);

    if let Some(max) = config_max {
	println!("The maximum is configured is {max}")
    }

}


