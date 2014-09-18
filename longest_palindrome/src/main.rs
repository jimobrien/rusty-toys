fn longest_palindrome<'a>(string: &'a str) -> &'a str {
  let string_len = string.len();
  let mut result:&str = "";

  let is_palindrome = |mut left: uint, mut right: uint| -> &'a str {
    while left as int >= 0 &&  right < string_len 
            && string.slice(left, left+1) == string.slice(right, right+1) {
      left -= 1;
      right += 1;
    }

    string.slice(left+1,right)
  };

  for i in range(0, string_len) {
    let odd_pal = is_palindrome(i-1, i+1);
    let even_pal = is_palindrome(i, i+1);

    if even_pal.len() > result.len() {
      result = even_pal;
    } 
    if odd_pal.len() > result.len() {
      result = odd_pal;
    }
  }

  result
}

#[test]
fn should_find_palindrom(){
  let word:&str = "ete";

  assert_eq!(is_palindrome(word), true);
}

#[test]
fn should_not_find_palindrom(){
  let word:&str = "eten";

  assert_eq!(is_palindrome(word), false);
}

#[cfg(not(test))]
fn main() {
  let sentance:&str  = "ass";
  println!("THE LONGEST PALINDROM IS: {}", longest_palindrome(sentance));
}
