pub use fin_model_gui::utils::clamp::clamp_one_pass::clamp;

fn parsed_num(n: u32) -> (u32, String) {
    (n, n.to_string())
}

fn main() {
    let min = 1980;
    let max = 2200;
    println!("{:?} , {:?}", parsed_num(1984), clamp("1984", min, max));
    println!("{:?} , {:?}", parsed_num(2025), clamp("2025", min, max));
    println!("{:?} , {:?}", parsed_num(2200), clamp("2200", min, max));
    println!("{:?} , {:?}", parsed_num(1980), clamp("1980", min, max));
    println!("{:?} , {:?}", parsed_num(1980), clamp("1979", min, max));
    println!("{:?} , {:?}", parsed_num(1999), clamp("1999", min, max));
    println!("{:?} , {:?}", parsed_num(2200), clamp("2540", min, max));
    println!("{:?} , {:?}", parsed_num(2200), clamp("99999", min, max));
    println!("{:?} , {:?}", parsed_num(198), clamp("193", min, max));
    println!("{:?} , {:?}", parsed_num(209), clamp("209", min, max));
}
