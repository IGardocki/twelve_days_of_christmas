fn main() {
    let mut song_verse = 0;

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let things_given = ["A partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds",
     "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing",
      "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    for number in 0..12 {
        let day = days[number];
        println!("On the {day} day of Christmas, my true love gave to me...");

        for verse in (0..=song_verse).rev() {
            let song_line = things_given[verse];
            println!("{song_line}");
        }
        song_verse+=1;

    }
}
