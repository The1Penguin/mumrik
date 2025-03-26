mod location;

struct State {
    user_agent: String,
    //cron_time: _,
    location: location::Location
}

fn main() {

    println!("{}", "Hello Nix flake!");
}
