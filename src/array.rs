

pub(crate) fn is_unique_chars_ascii(text: &str) -> bool {
    let mut arr = [false; 128];
    if text.len() > 128 {
        return false;
    }
    for ch in 0..text.len() {
        let char: u8 = text.bytes().nth(ch).unwrap();
        if arr[char as usize] == true {
            return false;
        }
        arr[char as usize] = true;
    }
    true
}

pub fn sort(text : &str) -> String {
    let mut arr =text.bytes().collect::<Vec<u8>>();
    arr.sort();
   String::from_utf8(arr).unwrap()


}
pub fn is_permutation_by_sort(s :&str,t:&str) -> bool{
    let s = s.to_lowercase();
    let t = t.to_lowercase();

    if s.len() != t.len() { return  false }
   return  sort(&*s).eq(&sort(&*t));

}
pub fn is_permutation_by_count(s :&str, t:&str) -> bool {
    let s = s.to_lowercase();
    let t = t.to_lowercase();

    if s.len() != t.len() { return  false }
    let mut letters = [0;128];
    let char_arr= (*s).chars().collect::<Vec<char>>();

    for char in char_arr.iter(){
        letters[*char as usize]+=1;
    }

    for el in 0..t.len(){
        let c = t.bytes().nth(el).unwrap();
        letters[c as usize]-=1;
        if letters[c as usize] < 0{ return  false}
    }
    true



}
pub fn  urlify(text : &str,true_length :u32) -> String{
let (mut space_count, mut index, _i)= (0, 0, 0);
    let mut text_vec = Vec::from(text);
    for i in 0..true_length{
        if text_vec[i as usize] == b' ' {
            space_count+=1;
        }
    }
    index = true_length + space_count * 2;
    if true_length < text.len() as u32 {
        text_vec[true_length as usize] =b'\0';
    }
         for i in true_length-1..0{
             if text_vec[true_length as usize] == b' ' {


                 text_vec[(index-1) as usize] = b'0';
                 text_vec[(index-2) as usize] = b'2';
                 text_vec[(index-3) as usize] = b'%';
                 index -= 3;
             } else {
                 text_vec[(index-1) as usize] = text_vec[i as usize];
                 index-=1;
             }
         }
    String::from_utf8(text_vec).unwrap()

        }




