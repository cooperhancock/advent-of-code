#![allow(unused)]
fn main() {
    let input_str: Vec<String> = input().as_str().split('\n').map(|x| x.to_owned()).collect();
    let boxes: Vec<Box> = input_str.into_iter().map(|x| 
        make_box(x.as_str().split('x').map(|y| y.parse::<u32>().unwrap()).collect())
        //x.as_str().split('x').map(|y| y.to_owned()).collect()
    ).collect();
    println!("{:?}", boxes.iter());
    println!("Part 1: {}", part1(boxes));
    //println!("Part 2: {}", part2(input_str);
}

fn part1(boxes: Vec<Box>) -> u32 {
    boxes.into_iter().fold(0, |acc, x| match x { 
        Box(l, w, h) => acc + (2*l*w) + (2*w*h) + (2*h*l) + [l*w, w*h, h*l].into_iter().min().unwrap() 
    })
}

fn part2(s: String) -> i32 {
    0
}

#[derive(Debug)]
struct Box(u32, u32, u32);

fn make_box(d: Vec<u32>) -> Box {
    Box(d[0], d[1], d[2])
}

fn input() -> String {
    String::from("1x1x1")
}
