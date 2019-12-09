pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Vec<char>>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image { width, height, layers: Vec::new() }
    }

    pub fn load(&mut self, buffer: &[u8]) {
        let data_count = self.width * self.height;
        let layer_count = buffer.len() / (self.width * self.height);
        for l in 0..layer_count {
            let mut layer = Vec::new();
            for d in 0..data_count {
                layer.push(buffer[l*data_count + d] as char);
            }
            self.layers.push(layer);
        }
    }

    pub fn char_count_layer(&self, ch: char, layer: usize) -> usize {
        let mut count = 0;
        for c in &self.layers[layer] {
            if *c == ch { count += 1; };
        }
        count
    }

    pub fn min_count_layer(&self, ch: char) -> usize {
        let mut min_layer = 0;
        let mut min_count = 99999;
        for layer in 0..self.layers.len() {
            let count = self.char_count_layer(ch, layer);
            if count < min_count { min_layer = layer; min_count = count; }
        }
        min_layer
    }

    pub fn decode(&self) -> Vec<char> {
        let data_count = self.width * self.height;
        let layer_count = self.layers.len();
        let mut result = self.layers[0].clone();
        for index in 0..data_count {
            for layer in 1..layer_count {
                if result[index] == '2'  { result[index] = self.layers[layer][index]; }
            }
        }
        result
    }

    pub fn decode_to_lines(&self) -> Vec<String> {
        let char_result = self.decode();
        let mut result = Vec::new();

        for row in 0..self.height {
            let mut s = String::new();
            for col in 0..self.width {
                match char_result[row * self.width + col] {
                    '0' => s.push(' '),
                    ch => s.push(ch)
                }
            }
            result.push(s);
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_part1_should_have_two_layers() {
        let mut image = Image::new(3, 2);  // width * height
        image.load("123456789012".as_bytes());

        assert_eq!(2, image.layers.len());
        assert_eq!(image.layers[0], vec!['1','2','3','4','5','6']);
        assert_eq!(image.layers[1], vec!['7','8','9','0','1','2']);
    }

    #[test]
    fn sample1_part1_should_have_one_zero_in_layer2() {
        let mut image = Image::new(3, 2);  // width * height
        image.load("123456789012".as_bytes());

        assert_eq!(1, image.char_count_layer('0', 1));
    }

    #[test]
    fn sample1_part1_should_have_most_8_in_layer2() {
        let mut image = Image::new(3, 2);  // width * height
        image.load("123456789012".as_bytes());

        assert_eq!(0, image.min_count_layer('8'))
    }

    #[test]
    fn sample1_part2_decode_image_as_0110() {
        let mut image = Image::new(2, 2);
        image.load("0222112222120000".as_bytes());

        let result = image.decode();

        assert_eq!(vec!['0', '1','1','0'], result);
    }
}