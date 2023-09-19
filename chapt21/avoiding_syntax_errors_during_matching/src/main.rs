


fn main() {
    /*
    macro_rules! complain {
        ($msg:expr) => {
            println!("Complaint filed: {}", $msg);
        };
        (user : $userid:tt , $msg:expr) => {
            println!("Complaint from user {}: {}", $userid, $msg);
        };
    }
    complain!(user: "jimb", "the AI lab's chatbots keep picking on me");
    */
    //////////
    //////////2
    macro_rules! complain {
        (msg : $msg:expr) => {
            println!("Complaint filed: {}", $msg);
        };
        (user : $userid:tt , msg : $msg:expr) => {
            println!("Complaint from user {}: {}", $userid, $msg);
        };
    }
    complain!(user: "jimb", msg: "the AI lab's chatbots keep picking on me");
    //////////////////////
}
