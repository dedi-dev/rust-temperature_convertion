use std::io;

fn main() {
    println!("Hello, welcome to temperature convertion app!");
    
    loop {
        println!("List unit convertion:");
        println!("1. Fahrenheit to Celcius");
        println!("2. Celcius to Fahrenheit");
        println!("3. Kelvin to Celcius");
        println!("4. Celcius to Kelvin");
        println!("0. Exit from app");
        println!("Please input number of Your choice. ");

        let mut select = String::new();
        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input not valid, please input number!");
                continue;
            }
        };

        match select {
            0 => {
                println!("Thank you for using this App");
                break;
            },
            1 | 2 | 3 | 4 => {
                println!("Please input number of degrees from unit that you have choice before. ");

                let mut degrees = String::new();
                io::stdin()
                    .read_line(&mut degrees)
                    .expect("Failed to read line");
                
                let degrees: f64 = match degrees.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input not valid, please input number!");
                        continue;
                    }
                };

                match select {
                    1 => {
                        let celcius_from_fahrenheit = fahrenheit_to_celcius(degrees);
                        println!("Convertion result : {degrees} degrees Fahrenheit is equal to {celcius_from_fahrenheit} degrees Celcius");
                    },
                    2 => {
                        let fahrenheit_from_celcius = celcius_to_fahrenheit(degrees);
                        println!("Convertion result : {degrees} degrees Celcius is equal to {fahrenheit_from_celcius} degrees Fahrenheit");
                    },
                    3 => {
                        let celcius_from_kelvin = kelvin_to_celcius(degrees);
                        println!("Convertion result : {degrees} degrees Kelvin is equal to {celcius_from_kelvin} degrees Celcius");
                    },
                    4 => {
                        let kelvin_from_celcius = celcius_to_kelvin(degrees);
                        println!("Convertion result : {degrees} degrees Celcius is equal to {kelvin_from_celcius} degrees Kelvin");
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 9.0/5.0) + 32.0
}

fn kelvin_to_celcius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn celcius_to_kelvin(celcius: f64) -> f64 {
    celcius + 273.15
}
