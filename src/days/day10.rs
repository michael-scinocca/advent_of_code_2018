struct Vector2 {
    x: i64,
    y: i64,
}

struct Pixel {
    position: Vector2,
    velocity: Vector2,
}

impl Pixel {
    fn new(pos_x: i64, pos_y: i64, vel_x: i64, vel_y: i64) -> Self {
        Pixel {
            position: Vector2 { x: pos_x, y: pos_y },
            velocity: Vector2 { x: vel_x, y: vel_y },
        }
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();

    let mut pixels = input
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.trim();

            line = line.trim_prefix("position=<");

            let end = line.find('>').unwrap();
            let pos_data = &mut line[..end]
                .split(',')
                .map(|p| p.trim().parse::<i64>().unwrap());

            line = line[end + 1..].trim();

            line = line.trim_prefix("velocity=<");

            let end = line.find('>').unwrap();
            let vel_data = &mut line[..end]
                .split(',')
                .map(|p| p.trim().parse::<i64>().unwrap());

            Pixel::new(
                pos_data.next().unwrap(),
                pos_data.next().unwrap(),
                vel_data.next().unwrap(),
                vel_data.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    for s in 0..15000 {
        let pos_x = pixels.iter().map(|p| p.position.x);
        let pos_y = pixels.iter().map(|p| p.position.y);

        let bounds = (
            pos_x.clone().min().unwrap(),
            pos_x.clone().max().unwrap(),
            pos_y.clone().min().unwrap(),
            pos_y.clone().max().unwrap(),
        );

        for pixel in &mut pixels {
            pixel.position.x += pixel.velocity.x;
            pixel.position.y += pixel.velocity.y;
        }
        
        if bounds.1 - bounds.0 > 75 {       
            continue;
        }

        println!("{}", s + 1);
        
        for y in bounds.2..=bounds.3 {
            for x in bounds.0..=bounds.1 {
                if pixels
                    .iter()
                    .any(|p| p.position.x == x && p.position.y == y)
                {
                    print!("*");
                } else {
                    print!(" ");
                }
            }

            println!();
        }

        println!();

        break;
    }
}

pub fn part2() {
    part1();
}
