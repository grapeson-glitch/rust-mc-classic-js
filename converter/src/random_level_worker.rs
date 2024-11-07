use crate::random::{self, Random};


//Creating the Distort struct
struct Distort {
    source: PerlinNoise,
    distort: PerlinNoise 
}

//Implicating Distort and building functions for it
impl Distort {

    fn new (source: PerlinNoise, distort: PerlinNoise) -> Self {
        Distort {
            source, 
            distort
        }
    }

    fn get_value (self, x: f64, y: f64) -> f64 {
        return self.source.get_value(x + self.distort.get_value(x, y), y);
    }
}

/*************************
  Perlin Noise Function
*************************/

struct PerlinNoise {
    noise_levels: Vec<ImprovedNoise>,
    levels: i32
}

impl PerlinNoise {
    fn new (random: f64, levels: i32) -> Self {

        let mut noise_levels: Vec<ImprovedNoise> = Vec::new();
        let levels: i32 = 8;

        for _ in 0..8 {
            noise_levels.push(ImprovedNoise::new(random));
        }

        PerlinNoise {
            noise_levels,
            levels
        }

    }

    fn get_value (self, x: f64, y: f64) -> f64 {

        let mut value: f64 = 0.0;
        let mut pow: f64 = 1.0;

        for i in 0..self.levels {
            value += self.noise_levels[i as usize].clone().get_value(x * pow, y * pow) / pow;
            pow /= 2.0;
        }

        return value;
    }
}

/*************************
  Improved Noise Function
*************************/

#[derive(Clone)]
struct ImprovedNoise {
    p: Vec<i32>
}

impl ImprovedNoise {
    fn new (random: f64) -> Self {
        let mut p: Vec<i32> = Vec::new();

        for i in 0..256 {
            p[i] = i as i32;
        }

        for i in 0..256 {
            //var j = random.nextInt(256 - i) + i;
            //var j = Math.round( Math.random() * 256-i ) + i;
            let mut j: i32 = (random * (256.0 - i as f64)).round() as i32 + i;
            let mut tmp: i32 = p[i as usize];
            p[i as usize] = p[j];
            p[j] = tmp;

            p[(i + 256) as usize] = p[i as usize];
        }

        ImprovedNoise {
            p
        }
    }

    fn fade_curve (d0: f64) -> f64 {
        return d0 * d0 * d0 * (d0 * (d0 * 6.0 - 15.0) + 10.0);
    }

    fn lerp (d0: f64, d1: f64, d2: f64) -> f64 {
        return d1 + d0 * (d2 - d1);
    }

    fn grad (i: i32, d0: f64, d1: f64, d2: f64) -> f64 {
        let d3: f64 = if (i & 15) < 8 {d0} else {d1};
        let d4: f64 = if i < 4 {d1} else {if i != 12 && i != 14 {d2} else {d0}};

        return (if (i & 1) == 0 {d3} else {-d3}) + (if (i & 2) == 0 {d4} else {-d4});
    }

    fn get_value (self, d0: f64, d1: f64) -> f64 {

        let mut d2: f64 = 0.0;
        let mut d3: f64 = d1;
        let mut d4: f64 = d0;
        let mut i: i32 = d0.floor() as i32 & 255;
        let mut j: i32 = d1.floor() as i32 & 255;

        const F0: f64 = 0.0;
        let k: i32 = F0.floor() as i32 & 255;

        d4 -= d4.floor();
        d3 -= d3.floor();
        d2 = 0.0 - F0.floor();
        let d5: f64 = ImprovedNoise::fade_curve(d4);
        let d6: f64 = ImprovedNoise::fade_curve(d3);
        let d7: f64 = ImprovedNoise::fade_curve(d2);
        let mut l: i32 = self.p[i as usize] + j;
        let i1: i32 = self.p[l as usize] + k;

        l = self.p[(l + 1) as usize] + k;
        i = self.p[(i + 1) as usize] + j;
        j = self.p[i as usize] + k;
        i = self.p[(i + 1) as usize] + k;

        return ImprovedNoise::lerp(d7, ImprovedNoise::lerp(d6, ImprovedNoise::lerp(d5, ImprovedNoise::grad(self.p[i1 as usize], d4, d3, d2), ImprovedNoise::grad(self.p[j as usize], d4 - 1.0, d3, d2)), ImprovedNoise::lerp(d5, ImprovedNoise::grad(self.p[l as usize], d4, d3 - 1.0, d2), ImprovedNoise::grad(self.p[i as usize], d4 - 1.0, d3 - 1.0, d2))), ImprovedNoise::lerp(d6, ImprovedNoise::lerp(d5, ImprovedNoise::grad(self.p[(i1 + 1) as usize], d4, d3, d2 - 1.0), ImprovedNoise::grad(self.p[(j + 1) as usize], d4 - 1.0, d3, d2 - 1.0)), ImprovedNoise::lerp(d5, ImprovedNoise::grad(self.p[(l + 1) as usize], d4, d3 - 1.0, d2 - 1.0), ImprovedNoise::grad(self.p[(i + 1) as usize], d4 - 1.0, d3 - 1.0, d2 - 1.0))));
    }
}
