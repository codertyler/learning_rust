pub fn run() {
  //Print to console
  println!("hello from the print.rs file");

  //Basic formatting
  println!("Number: {}", 1);

  //Positional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Tyler", "LHL", "code" );

  //Named arguments
  println!("{name} likes to play {activity}", name="Tyler", activity="ping pong")
}