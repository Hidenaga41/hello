fn main() {
    // let x: i32 = 10;
    // let y = 20;
    // let z = mul(x, y);
    // println!("z = {z}");
    let player: u16 =
    1 |
    (1 << 1) |
    (568 << 2);

    if player & 1 != 0 {
        println!("毒状態")
    }

    if player & (1 << 1) !=0 {
        println!("攻撃力アップ状態")
    }
}

fn mul(x: i32, y: i32) -> i32{
    x*y


}

