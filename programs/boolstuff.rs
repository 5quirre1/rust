fn main() {
    // friend wants
    let friend_wants_mcdonalds: bool = false;
    let friend_wants_wendys:bool = false;
    let friend_wants_wow:bool = true;

    // you wants
    let u_wants_mcdonalds: bool = false;
    let u_wants_wendys:bool = false;
    let u_wants_wow:bool = true;

    // all friend
    let friend_wants_place: [bool; 3] = [
        friend_wants_mcdonalds,
        friend_wants_wendys,
        friend_wants_wow,
    ];

    // all u
    let u_want_place: [bool;3] = [
        u_wants_mcdonalds,
        u_wants_wendys,
        u_wants_wow,
    ];

    // check
    let mut check:bool = false;
    for i in 0..friend_wants_place.len() {
        if friend_wants_place[i] && u_want_place[i] {
            check = true;
            break;
        }
    }
    // ok
    if check {
        println!("you and friend wanted to go to atleast one of the same places!!");
    } else {
        println!("u and friend didn't want to go to same places.");
    }
}
