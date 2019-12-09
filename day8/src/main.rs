use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let width = 25usize;
    let height = 6usize;

    let nums: Vec<u32> = from_file("input.txt")?;
    let layers = nums.chunks(width * height);

    let pixels: Vec<Vec<u32>> = layers.map(|layer| layer.collect()).collect();

    let layer_w_fewest_zeroes = layers.min_by(|layer1, layer2| {
        let layer1_zeroes = layer1.iter().filter(|pix| *pix == &0).count();
        let layer2_zeroes = layer2.iter().filter(|pix| *pix == &0).count();
        layer1_zeroes.cmp(&layer2_zeroes)
    }).unwrap();

    let layer_ones = layer_w_fewest_zeroes.iter().filter(|pix| *pix == &1).count();
    let layer_twos = layer_w_fewest_zeroes.iter().filter(|pix| *pix == &2).count();
    
    println!("Part 1: {}", layer_ones * layer_twos);
    Ok(())
}

fn from_file(path: &str) -> Result<Vec<u32>> {
    let raw: String = fs::read_to_string(path).unwrap();
    let numbers: Vec<u32> = raw.trim().chars().filter_map(|character| character.to_digit(10)).collect();
    Ok(numbers)
}

fn shape_layer(layer: Vec<u32>, width: usize) -> Vec<Vec<u32>> {
    
}
