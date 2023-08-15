
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    // let mut a = new([1,2,3,4,5]);
    // let mut b = a;
    // b[0] = 6 ;

    let mut user1 = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    // when instance is mutable, you can modify the instance
    user1.email = String::from("anotherEmail@example.com");

    let user2 = User{
        // email: String::from("another@example.com"),

        sign_in_count: 2,
        ..user1
    };

    // Tuple struct
    let black = Color(0, 0, 0);

    // compile doesn't work cause, user1 has moved to user2(ownership)
    // but user1.active(boolean), user1.sign_in_count are fine cause they are implementing Copy trait
    // print!("{}",user1.username); // not complied
    print!("{}", user2.email); // compiled
    
    

    // Rust doesn't allow us to mark only certain fields as mutable
    

    

    

    
    // let s2 = String::from("hello");     
    

    // let s3 = takes_and_gives_back(s2);

    // print!("{}", s3);
}

fn build_user(email:String, username: String)-> User{
    User{
        active: true,
        username,
        email,
        sign_in_count:1
    }
}

fn takes_ownership(some_str: String){
    print!("{}", some_str);

}
fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_str = String::from("yours");
    some_str
}
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
