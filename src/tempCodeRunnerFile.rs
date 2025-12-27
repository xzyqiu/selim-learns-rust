    let max = cmp::max(cmp::max(red, green), blue);
    let min = cmp::min(cmp::min(red, green), blue);
    let delta = max - min;
    
    // Calculate saturation and brightness
    let brightness = max;
    let saturation = if max == 0 { 0 } else { (delta * 255) / max };
    
    // Calculate hue
    let hue = if delta == 0 {
        0
    } else if max == red {
        (60 * (((green as i32 - blue as i32) * 60) / delta as i32)) % 360
    } else if max == green {
        (60 * (((blue as i32 - red as i32) * 60) / delta as i32)) + 120
    } else {
        (60 * (((red as i32 - green as i32) * 60) / delta as i32)) + 240
    };
    
    // Identify color based on HSB values
    if brightness < 30 {
        println!("{}", "The color is Black".black());
    } else if brightness > 200 && saturation < 30 {
        println!("{}", "The color is White".white());
    } else if saturation < 50 {
        println!("{}", "The color is Gray".white());
    } else if hue >= 330 || hue < 30 {
        println!("{}", format!("The color is Red (H:{}° S:{}% B:{}%)", hue.max(0), saturation, (brightness * 100) / 255).red());
    } else if hue >= 30 && hue < 90 {
        println!("{}", format!("The color is Yellow (H:{}° S:{}% B:{}%)", hue, saturation, (brightness * 100) / 255).yellow());
    } else if hue >= 90 && hue < 150 {
        println!("{}", format!("The color is Green (H:{}° S:{}% B:{}%)", hue, saturation, (brightness * 100) / 255).green());
    } else if hue >= 150 && hue < 210 {
        println!("{}", format!("The color is Cyan (H:{}° S:{}% B:{}%)", hue, saturation, (brightness * 100) / 255).cyan());
    } else if hue >= 210 && hue < 270 {
        println!("{}", format!("The color is Blue (H:{}° S:{}% B:{}%)", hue, saturation, (brightness * 100) / 255).blue());
    } else {
        println!("{}", format!("The color is Magenta (H:{}° S:{}% B:{}%)", hue, saturation, (brightness * 100) / 255).magenta());
    }
}