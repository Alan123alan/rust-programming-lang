fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics = ["A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"];

    for i in 0..days.len(){
        println!("On the {} day of Christmas, my true love sent to me",days[i]);
        for j in (0..=i).rev(){
            println!("{}",lyrics[j]);
        } 
    }
}
