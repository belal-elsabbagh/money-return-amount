fn main() {
    let amount: u32 = get_amount();
    const DENOMINATORS: [u8; 7] = load_denominators();
    let money_return: Vec<u8> = min_money_return(amount, DENOMINATORS);
    println!("{:?}", money_return);
}

const fn load_denominators() -> [u8; 7] {
    return [200, 100, 50, 20, 10, 5, 1];
}

fn get_amount() -> u32 {
    return std::env::args()
        .nth(1)
        .expect("Invalid amount of money entered")
        .parse()
        .unwrap();
}

fn min_money_return(mut amount: u32, denominators: [u8; 7]) -> Vec<u8> {
    let mut money_back: Vec<u8> = Vec::new();
    while amount > 0 {
        let mut found: bool = false;
        for d in &denominators {
            let d32 = *d as u32;
            if amount >= d32 {
                amount -= d32;
                money_back.push(*d);
                found = true;
                break;
            }
        }
        if !found{
            panic!("Cannot subdivide {}", amount.to_string());
        }
    }
    return money_back;
}
