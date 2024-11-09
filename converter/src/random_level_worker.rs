use crate::random::{self, Random};
use std::str::FromStr;


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

    fn get_value (&self, x: f64, y: f64) -> f64 {
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

    fn get_value (&self, x: f64, y: f64) -> f64 {

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
pub struct ImprovedNoise {
    p: Vec<i32>
}

impl ImprovedNoise {
    pub fn new (random: f64) -> Self {
        let mut p: Vec<i32> = Vec::new();

        for i in 0..256 {
            p[i] = i as i32;
        }

        for i in 0..256 {
            //var j = random.nextInt(256 - i) + i;
            //var j = Math.round( Math.random() * 256-i ) + i;
            let mut j: i32 = (random * (256.0 - i as f64)).round() as i32 + i;
            let mut tmp: i32 = p[i as usize];
            p[i as usize] = p[j as usize];
            p[j as usize] = tmp;

            p[(i + 256) as usize] = p[i as usize];
        }

        ImprovedNoise {
            p
        }
    }

    pub fn fade_curve (&self, d0: f64) -> f64 {
        return d0 * d0 * d0 * (d0 * (d0 * 6.0 - 15.0) + 10.0);
    }

    pub fn lerp (&self, d0: f64, d1: f64, d2: f64) -> f64 {
        return d1 + d0 * (d2 - d1);
    }

    pub fn grad (&self, mut i: i32, d0: f64, d1: f64, d2: f64) -> f64 {
        i &= 15;
        let d3: f64 = if i < 8 {d0} else {d1};
        let d4: f64 = if i < 4 {d1} else {if i != 12 && i != 14 {d2} else {d0}};

        return (if (i & 1) == 0 {d3} else {-d3}) + (if (i & 2) == 0 {d4} else {-d4});
    }

    pub fn get_value (&self, d0: f64, d1: f64) -> f64 {

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
        let d5: f64 = self.fade_curve(d4);
        let d6: f64 = self.fade_curve(d3);
        let d7: f64 = self.fade_curve(d2);
        let mut l: i32 = self.p[i as usize] + j;
        let i1: i32 = self.p[l as usize] + k;

        l = self.p[(l + 1) as usize] + k;
        i = self.p[(i + 1) as usize] + j;
        j = self.p[i as usize] + k;
        i = self.p[(i + 1) as usize] + k;

        return self.lerp(d7, self.lerp(d6, self.lerp(d5, self.grad(self.p[i1 as usize], d4, d3, d2), self.grad(self.p[j as usize], d4 - 1.0, d3, d2)), self.lerp(d5, self.grad(self.p[l as usize], d4, d3 - 1.0, d2), self.grad(self.p[i as usize], d4 - 1.0, d3 - 1.0, d2))), self.lerp(d6, self.lerp(d5, self.grad(self.p[(i1 + 1) as usize], d4, d3, d2 - 1.0), self.grad(self.p[(j + 1) as usize], d4 - 1.0, d3, d2 - 1.0)), self.lerp(d5, self.grad(self.p[(l + 1) as usize], d4, d3 - 1.0, d2 - 1.0), self.grad(self.p[(i + 1) as usize], d4 - 1.0, d3 - 1.0, d2 - 1.0))));
    }
}


struct RandomLevel {

    progress_string: String,
    progress_percent: i32,
    progress_tiles: Vec<u8>, 	
    x_size: i32,
    y_size: i32,
    z_size: i32,
    random: Random,
    rand: f64,
    tiles: Vec<u8>,
    fill_queue: Vec<i32>

}

impl RandomLevel {

    pub fn new (self, seed: i32, x_size: i32, z_size: i32, mut y_size: i32) -> RandomLevel {
        let progress_string: String = String::from("");
        let progress_percent: i32 = 0;
        let progress_tiles: Vec<u8> = Vec::new();

        y_size = 64;

        let mut random: Random = Random::new(seed);
        let rand: f64 = random.nextFloat();
        let tiles: Vec<u8> = Vec::new();
        let fill_queue: Vec<i32> = Vec::new();

        RandomLevel {
            progress_string,
            progress_percent,
            progress_tiles,
            x_size,
            y_size,
            z_size,
            random,
            rand,
            tiles,
            fill_queue
        }

    }

    //grow
    pub fn grow (&mut self, aint: Vec<f64>) {
        let i: i32 = self.x_size;
        let j: i32 = self.z_size;
        let k: i32 = self.y_size;
        let perlin_noise: PerlinNoise = PerlinNoise::new(self.rand, 8);
        let perlin_noise1: PerlinNoise = PerlinNoise::new(self.rand, 8);

        for l in 0..i {
            //this.progress(l * 100 / (this.xSize - 1));
            self.progress_percent = l * 100 / (self.x_size - 1);
            //self.postMessage(progress);

            for i1 in 0..j {
                let flag: bool = perlin_noise.get_value(l as f64, i1 as f64) > 8.0;
                let flag1: bool = perlin_noise1.get_value(l as f64, i1 as f64) > 12.0;
                let j1: i32;
                //var k1 = parseInt( ((j1 = parseInt(aint[l + i1 * i],10)) * this.zSize + i1) * this.xSize + l, 10);
                j1 = aint[(l + i1 * i) as usize] as i32;
                let k1: i32 = ((j1 * self.z_size + i1) * self.x_size + l) as i32;
                let l1: i32;

                // 7 waterid
                //if (((l1 = parseInt(this.tiles[((j1 + 1) * this.zSize + i1) * this.xSize + l],10) & 255) == 7) && j1 <= k / 2 - 1 && flag1) {
                l1 = (self.tiles[(((j1 + 1) * self.z_size + i1) * self.x_size + l) as usize]) as i32 & 255;
                if (l1 == 7) && j1 <= k / 2 - 1 && flag1 {

                    self.tiles[k1 as usize] = 12;//(byte) Tile.gravel.id;
                }

                if l1 == 0 {
                    let mut i2: i32 = 1;//Tile.grass.id;

                    if j1 <= k / 2 - 1 && flag {
                        i2 = 11;//Tile.sand.id;
                    }

                    self.tiles[k1 as usize] = i2 as u8;
                }
            }
        }
    }

    //melt
    pub fn melt (&mut self) {
        let mut i: i32 = 0;
        let j: i32 = self.x_size * self.z_size * self.y_size / 10000;

        for k in 0..j {
            if k % 100 == 0 {
                self.progress_percent = k * 100 / (j - 1);
                //self.postMessage(progress);
            }

            let extray: i32 = 16;
            let l: i32 = self.random.nextInt(self.x_size);
            let i1: i32 = self.random.nextInt(self.y_size / 2 - 4) + extray;
            let j1: i32 = self.random.nextInt(self.z_size);

            if self.tiles[((i1 * self.z_size + j1) * self.x_size + l) as usize] == 0 {
                i += 1;
                self.flood_fill(l, i1, j1, 0, 17);

            }
        }
    }

    //plant
    pub fn plant (&mut self, aint: Vec<f64>) {
        let i: i32 = self.x_size;
        let j: i32 = self.x_size * self.z_size / 4000;

        for k in 0..j {
            self.progress_percent = k * 100 / (j - 1);
            //self.postMessage(progress);

            let l: i32 = self.random.nextInt(self.x_size);
            let i1: i32 = self.random.nextInt(self.z_size);

            for _ in 0..20 {
                let mut k1: i32 = l;
                let mut l1: i32 = i1;

                for _ in 0..20 {
                    k1 += self.random.nextInt(6) - self.random.nextInt(6);
                    l1 += self.random.nextInt(6) - self.random.nextInt(6);

                    if k1 >= 0 && l1 >= 0 && k1 < self.x_size && l1 < self.z_size {
                        let j2: f64 = aint[(k1 + l1 * i) as usize] + 1.0;//No idea if this is supposed to be a float or an int...
                        let k2: i32 = self.random.nextInt(3) + 4;
                        let mut flag: bool = true;

                        let mut l2: f64 = j2;
                        let mut i3: f64;
                        let mut j3: f64;

                        while l2 <= j2 + 1.0 + k2 as f64 {
                            let mut b0: i32 = 1;

                            if l2 >= j2 + 1.0 + k2 as f64 - 2.0 {
                                b0 = 2;
                            }

                            i3 = (k1 - b0) as f64;
                            while i3 <= (k1 + b0) as f64 && flag {
                                j3 = (l1 - b0) as f64;
                                while j3 <= (l1 + b0) as f64 && flag {
                                    if i3 >= 0.0 && l2 >= 0.0 && j3 >= 0.0 && i3 < self.x_size as f64 && l2 < self.y_size as f64 && j3 < self.z_size as f64 {
                                        if (self.tiles[((l2 * self.z_size as f64 + j3) * self.x_size as f64 + i3) as usize] & 255) != 0 {
                                            flag = false;
                                        }
                                    } else {
                                        flag = false;
                                    }
                                    j3 += 1.0;
                                }
                                i3 += 1.0; 
                            }
                            l2 += 1.0;
                        }

                        if flag {
                            l2 = (j2 * self.z_size as f64 + l1 as f64) * self.x_size as f64 + k1 as f64;
                            if (self.tiles[(((j2 - 1.0) * self.z_size as f64 + l1 as f64) * self.x_size as f64 + k1 as f64) as usize] & 255) == 1 && j2 < self.y_size as f64 - k2 as f64 - 1.0 {
                                self.tiles[(l2 - 1.0 * self.x_size as f64 * self.z_size as f64) as usize] = 3;//(byte) Tile.dirt.id;

                                i3 = j2 - 3.0 + k2 as f64;
                                while i3 <= j2 + k2 as f64 {

                                    j3 = i3 - (j2 + k2 as f64);
                                    let k3: i32 = (1.0 - j3 / 2.0) as i32;
                                    let mut l3: i32 = k1 - k3;

                                    while l3 <= k1 + k3 {

                                        let i4: i32 = (l3 - k1) as i32;
                                        let mut j4: i32 = l1 - k3;

                                        while j4 <= l1 + k3 {

                                            let k4: i32 = (j4 - l1) as i32;

                                            if i32::abs(i4) != k3 || i32::abs(k4) != k3 || self.random.nextInt(2) != 0 && j3 != 0.0 {
                                                self.tiles[((i3 * self.z_size as f64 + j4 as f64) * self.x_size as f64 + l3 as f64) as usize] = 14;//(byte) Tile.leaves.id;
                                            }
                                            j4 += 1;
                                        }
                                        l3 += 1;
                                    }
                                    i3 += 1.0;
                                }

                                i3 = 0.0;
                                while i3 < k2 as f64 {
                                    self.tiles[(l2 + i3 * self.x_size as f64 * self.z_size as f64) as usize] = 13;//(byte) Tile.treeTrunk.id;
                                    i3 += 1.0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn placeOre (&mut self, tile: u8, j: i32, k: i32, mut l: i32) {
        l = self.x_size;
        let i1: i32 = self.z_size;
        let j1: i32 = self.y_size;
        let k1: i32 = l * i1 * j1 / 256 / 64 * j / 100;

        for l1 in 0..k1 {
            self.progress_percent = l1 * 100 / (k1 - 1) / 4 + k * 100 / 4;
            //self.postMessage(progress);

            let mut f: f64 = self.random.nextFloat() * l as f64;
            let mut f1: f64 = self.random.nextFloat() * j1 as f64;
            let mut f2: f64 = self.random.nextFloat() * i1 as f64;
            let i2: i32 = ((self.random.nextFloat() + self.random.nextFloat()) * 75.0 * j as f64 / 100.0) as i32; //parseInt()
            let mut f3: f64 = self.random.nextFloat() * 3.141592653589793 * 2.0;
            let mut f4: f64 = 0.0;
            let mut f5: f64 = self.random.nextFloat() * 3.141592653589793 * 2.0;
            let mut f6: f64 = 0.0;

            for j2 in 0..i2 {
                f =  f + f64::sin(f3) * f64::cos(f5);
                f2 = f2 + f64::cos(f3) * f64::cos(f5);
                f1 = f1 + f64::sin(f5);
                f3 += f4 * 0.2;
                f4 *= 0.9;
                f4 = f4 + (self.random.nextFloat() - self.random.nextFloat());
                f5 = (f5 + f6 * 0.5) * 0.5;
                f6 *= 0.9;
                f6 = f6 + (self.random.nextFloat() - self.random.nextFloat());
                let f7: f64 = f64::sin(j2 as f64 * 3.141592653589793 / i2 as f64) * j as f64 / 100.0 + 1.0;

                let mut k2:f64  = (f - f7).round();

                while k2 <= (f + f7).round() {

                    let mut l2: f64 = (f1 - f7).round();

                    while l2 <= (f1 + f7).round() {

                        let mut i3: f64 = (f2 - f7).round();

                        while i3 <= (f2 + f7).round() {
                            let f8: f64 = k2 - f;
                            let f9: f64 = l2 - f1;
                            let f10: f64 = i3 - f2;

                            if f8 * f8 + f9 * f9 * 2.0 + f10 * f10 < f7 * f7 && k2 >= 1.0 && l2 >= 1.0 && i3 >= 1.0 && k2 < self.x_size as f64 - 1.0 && l2 < self.y_size as f64 - 1.0 && i3 < self.z_size as f64 - 1.0 {
                                let j3: i32 = ((l2 * self.z_size as f64 + i3) * self.x_size as f64 + k2) as i32; //parseInt()

                                //if (this.tiles[j3] == Tile.rock.id) {
                                if self.tiles[j3 as usize] == 2 {
                                    self.tiles[j3 as usize] = tile;
                                }
                            }
                            i3 += 1.0;
                        }
                        l2 += 1.0;
                    }
                    k2 += 1.0;
                }
            }
        }
    }

    pub fn flood_fill (&mut self, xc: i32, yc: i32, zc: i32, unused: u8, tile: u8) -> i32 {
        let mut w_bits: i32 = 1;
        let mut h_bits: i32 = 1;

        while (1 << w_bits) < self.x_size { w_bits += 1; }
        while (1 << h_bits) < self.y_size { h_bits += 1; }

        let z_mask: i32 = self.z_size - 1;
        let x_mask: i32 = self.x_size - 1;
        let mut count: i32 = 1;

        self.fill_queue[0] = ((yc << h_bits) + zc << w_bits) + xc;

        let mut k2: i32 = 0;

        let offset: i32 = self.x_size * self.z_size;

        while count > 0 {

            count -= 1;

            let mut val: i32 = self.fill_queue[count as usize];

            let z: i32 = val >> w_bits & z_mask;
            let l2: i32 = val >> w_bits + h_bits;

            let mut i3: i32 = 0;
            let mut j3: i32 = 0;

            i3 = val & x_mask;
            j3 = i3;

            while i3 > 0 && self.tiles[(val - 1) as usize] == 0 {
                i3 -= 1;
                val -= 1;
            }

            while j3 < self.x_size && self.tiles[(val + j3 - i3) as usize] == 0 {
                j3 += 1;
            }

            let k3: i32 = val >> w_bits & z_mask;
            let l3: i32 = val >> w_bits + h_bits;

            if k3 != z || l3 != l2 {
                //System.out.println("hoooly fuck");
                //console.log("hoooly fuck")
                println!("hoooly fuck");
            }

            let mut flag: bool = false;
            let mut flag1: bool = false;
            let mut flag2: bool = false;

            k2 += j3 - i3;

            i3 = i3;

            while i3 < j3 {

                self.tiles[val as usize] = tile;
                let mut flag3: bool;

                if z > 0 {

                    flag3 = self.tiles[(val - self.x_size) as usize] == 0;

                    if flag3 && !flag {

                        count += 1;
                        self.fill_queue[count as usize] = val - self.x_size;

                    }

                    flag = flag3;

                }

                if z < self.z_size - 1 {

                    flag3 = self.tiles[(val + self.x_size) as usize] == 0;

                    if flag3 && !flag1 {

                        count += 1;
                        self.fill_queue[count as usize] = val + self.x_size;

                    }

                    flag1 = flag3;

                }

                if l2 > 0 {
                    let b2: u8 = self.tiles[(val - offset) as usize];

                    //if (( tile == Tile.lava.id || tile == Tile.calmLava.id) && (b2 == Tile.water.id || b2 == Tile.calmWater.id)) {
                    if ( tile == 17) && (b2 == 7) {
                        self.tiles[(val - offset) as usize] = 2;//Tile.rock.id;
                    }

                    flag3 = b2 == 0;
                    if flag3 && !flag2 {

                        count += 1;
                        self.fill_queue[count as usize] = val - offset;

                    }

                    flag2 = flag3;
                }
                val += 1;
                i3 += 1;
            }
        }
        return k2;
    }
}

