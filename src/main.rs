fn main() {
    let width: u32 = 800;
    let height: u32 = 800;

    print!("{}", format!("P3\n{} {}\n255\n", width, height));
    for i in 0..width {
        for j in 0..height {
            let red = 255.999 * i as f32 / (width - 1) as f32;
            let green = 255.999 * j as f32 / (height - 1) as f32;
            let blue = 0;

            println!("{}", format!("{} {} {}", red as u8, green as u8, blue as u8));
        }
    }
}
