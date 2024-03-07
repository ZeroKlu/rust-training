fn main() {
    let valid_scales: [String;2] = ["F".to_string(), "C".to_string()];
    let temp: f64 = get_double("Enter temperature in degrees:");
    let mut scale = String::new();
    while !valid_scales.contains(&scale) {
        scale = get_string("Enter scale (F or C):", true);
    }
    let converted_scale = if scale == "F" {"C"} else {"F"};
    let converted_temp = convert(temp, &scale);

    println!("{}°{} = {}°{}", temp, scale, converted_temp, converted_scale);
}

fn convert(temp: f64, scale: &String) -> f64 {
    if scale.to_uppercase() == "F".to_string() {
        return f_to_c(temp);
    }
    return c_to_f(temp);
}

fn f_to_c(temp: f64) -> f64 {
    return ((temp - 32.0) * 5.0 / 9.0 * 100.0).round() / 100.0;
}

fn c_to_f(temp: f64) -> f64 {
    return (temp * 9.0 / 5.0 + 32.0 * 100.0).round() / 100.0;
}

fn get_double(message: &str) -> f64 {
    println!("{}", message);
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    return match line.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => 0.0
    };
}

fn get_string(message: &str, upper: bool) -> String {
    println!("{}", message);
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    return if upper {line.to_uppercase()} else {line};
}
