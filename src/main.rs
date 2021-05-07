fn main() -> std::io::Result<()> {
  use std::io::{Write, stdin};
  use std::fs::File;

  println!("What's the name of the file do you want to write to, including the extension:");
  let mut filename = String::new();
  stdin().read_line(&mut filename).expect("Did not enter a correct string");
  filename.pop();

  println!("What do you want to write:");
  let mut contents = String::new();
  stdin().read_line(&mut contents).expect("Did not enter a correct string");
  contents.pop();
  
  let mut file = File::create(filename.as_str())?;

  let mut vec_of_lines = Vec::<String>::new();
  for _i in 0..6 {
    vec_of_lines.push("".to_string());
  }
  for character in 0..contents.chars().count() {
    for i in 0..6 {
      let mut line = get_letter(contents.chars().nth(character).unwrap()).lines().nth(i).unwrap().to_string();
      
      loop {
        if line.chars().count() <= 12 {
          line.push(' ');
        } else {
          break;
        }
      }

      if vec_of_lines.len() > i && vec_of_lines[i].chars().count() > 0 && vec_of_lines[i].chars().last().unwrap() == '\n' {
        vec_of_lines[i].pop();
      }

      vec_of_lines[i] = [vec_of_lines[i].clone(), line].concat();

      vec_of_lines[i].push('\n');
    }
  }

  for index in vec_of_lines {
    write!(file, "{}", index);
  }
  
  Ok(())
}

fn get_letter(letter: char) -> String {
match letter {
'a' => 
"     __
    /  \\
   / /\\ \\
  / /__\\ \\
 / .----. \\
/_/      \\_\\".to_string(),

'b' =>
" ______
|  ___ \\
| |___| |
|  ___ <
| |___| |
|______/".to_string(),

'c' =>
"   ____
  / ___|
 / /
| |
 \\ \\___
  \\____|".to_string(),

'd' =>
" ____
|  _ \\
| | | |
| | | |
| |_| |
|____/".to_string(),

'e' =>
" ______
|  ____|
| |____
|  ____|
| |____
|______|".to_string(),

'f' =>
" ______
|  ____|
| |____
|  ____|
| |
|_|".to_string(),

'g' =>
"   ____
  / ___|
 / /  _
| |  | |
 \\ \\_| |
  \\____|".to_string(),

'h' =>
" _     _
| |   | |
| |___| |
|  ____ |
| |   | |
|_|   |_|".to_string(),

'i' =>
" _
| |
| |
| |
| |
|_|".to_string(),

'j' =>
"    _
   | |
   | |
 _ | |
| || |
|____|".to_string(),

'k' =>
" _   _
| | / /
| |/ /
|   \\
| |\\ \\
|_| \\_\\".to_string(),

'l' =>
" _
| |
| |
| |
| |____
|_____|".to_string(),

'm' =>
" __  __
|  \\/  |
| |\\/| |
| |  | |
| |  | |
|_|  |_|".to_string(),

'n' =>
" __  __
|  \\ | |
| |\\\\| |
| | \\  |
| |  | |
|_|  |_|".to_string(),

'o' =>
"   _____
  / ___ \\
 / /   \\ \\
| |     | |
 \\ \\___/ /
  \\_____/".to_string(),

'p' =>
" ___
|  _ \\
| |_| |
|  __/
| |
|_|".to_string(),

'q' =>
"   _____
  / ___ \\
 / /   \\ \\
| |     | \\
 \\ \\___/  |
  \\_____/|_\\".to_string(),

'r' =>
" ___
|  _ \\
| |_| |
|  __ \\
| |  | |
|_|  |_|".to_string(),

's' =>
"   ___
 / __|
| \\_
 \\_ \\
 _/  /
|___/ ".to_string(),

't' =>
" _____
|_   _|
  | |
  | |
  | |
  |_|".to_string(),

'u' =>
" _   _
| | | |
| | | |
| | | |
| |_| |
|_____|".to_string(),

'v' =>
"__      __
\\ \\    / /
 \\ \\  / /
  \\ \\/ /
   \\  /
    \\/".to_string(),

'w' =>
" _       _
| |     | |
| |     | |
| |  _  | |
| |_| |_| |
|_________|".to_string(),

'x' =>
"__    __
\\ \\  / /
 \\ \\/ /
  |  |
 / /\\ \\
/_/  \\_\\".to_string(),

'y' =>
"__    __
\\ \\  / /
 \\ \\/ /
  |  |
  |  |
  |__|".to_string(),

'z' =>
" ______
|___  /
   / /
  / /
 / /___
|______|".to_string(),

_ => "not a character that has been made yet".to_string()
}
}