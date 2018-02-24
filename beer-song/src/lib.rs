const NO_BEER: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const ONE_BEER: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const TWO_BEERS: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(i: i32) -> String {
    sing(i, i)
}

pub fn sing(start: i32, stop: i32) -> String {
    let mut song: String = String::new();
    let mut i: i32 = 0;
    loop {
        let cur: i32 = start - i;
        match cur {
            0 => song.push_str(NO_BEER),
            1 => song.push_str(ONE_BEER),
            2 => song.push_str(TWO_BEERS),
            _ => song.push_str(&format!("{b} bottles of beer on the wall, {b} bottles of beer.\nTake one down and pass it around, {bn} bottles of beer on the wall.\n", b = cur, bn = cur - 1)),  
        }
        i = i + 1;
        if i > start - stop {
            break;
        } else {
            song.push_str("\n");
        }
    }
    song
}