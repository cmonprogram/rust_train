fn main() {
    //Конвертация температур между значениями по Фаренгейту к Цельсию.
    println!("Enter: %number% [CEL/FAR]");
    loop{
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)
        .expect("Failed to read line");
        let args : Vec<&str> = input.split(' ').collect();
        if args.len() < 2 {
            println!("Wrong data");
            continue;
        };

        let input_temperature : f32 = args[0].parse().expect("Wrong data!");
        let out_temperature : f32;
        let type_temperature : String = args[1].to_string();

        match type_temperature.as_str() {
            "FAR\n" => {            
                out_temperature = (input_temperature  - 32.0) / (9.0 / 5.0);
                println!("{} CEL", out_temperature);
            },
            "CEL\n" => {            
                out_temperature = input_temperature * (9.0 / 5.0) + 32.0;
                println!("{} FAR", out_temperature);
            },  
            _ =>{
                println!("Wrong data");
            }         
        }
    }
}
