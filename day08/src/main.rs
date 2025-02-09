use std::fs;

fn parse(image_data: &str) -> Vec<u8> {
    image_data.bytes().map(|c| c - 48).collect()
}

fn count(xs: &[u8], x: u8) -> usize {
    xs.iter().filter(|y| **y == x).count()
}

fn part1(pixels: &[u8], width: usize, height: usize) -> usize {
    let layers = pixels.chunks(width * height);
    let layer = layers.min_by_key(|&layer| count(layer, 0)).unwrap();
    count(layer, 1) * count(layer, 2)
}

fn combine(image: &mut [u8], layer: &[u8]) {
    for (a, b) in image.iter_mut().zip(layer) {
        *a = if *a == 2 { *b } else { *a };
    }
}

fn render(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let size = width * height;
    let mut image = pixels[..size].to_vec();
    for layer in pixels.chunks(size).skip(1) {
        combine(&mut image, layer);
    }
    image
}

fn display(image: &[u8], width: usize) {
    for row in image.chunks(width) {
        let line: String = row
            .iter()
            .map(|x| if *x == 0 { ' ' } else { '#' })
            .collect();
        println!("{line}");
    }
}

fn main() {
    let image_data = fs::read_to_string("input").unwrap();
    let pixels = parse(image_data.trim());
    let (width, height) = (25, 6);

    println!("{}", part1(&pixels, width, height));

    let image = render(&pixels, width, height);
    display(&image, width);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let image_data = "123000112200111220";
        let pixels = parse(image_data);
        assert_eq!(part1(&pixels, 3, 2), 6);
    }

    #[test]
    fn test_part2() {
        let image_data = "0222112222120000";
        let pixels = parse(image_data);
        assert_eq!(&render(&pixels, 2, 2), &[0, 1, 1, 0]);
    }
}
