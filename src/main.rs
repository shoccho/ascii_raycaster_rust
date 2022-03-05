use std::f64::consts::PI;
use std::{f64, time};
const WIDTH: usize = 256;
const HEIGHT: usize = 64;
static DT: f64 = 0.0174533;

static SCALE: f64 = 100.0;
static FOV: f64 = 256.0;
const MAP_W: usize = 8;
const MAP_SIZE :usize = 64;
#[derive(Debug)]
struct Player {
    x: f64,
    y: f64,
    a: f64,
}

static PALLET: [char; 7] = ['.', '*', 'x', '%', '$', '#', '@'];

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}
fn interpolate(value: f64, x: f64, y: f64, p: f64, q: f64) -> f64 {
    p + ((value - x) / (y - x)) * (q - p)
}
fn doshit(player: &Player, map: &[usize; 64]) -> ([[char; WIDTH]; HEIGHT], f64) {
    //buffer
    let mut buffer = [[' '; WIDTH]; HEIGHT];

    let mut ray_x: f64 = 0.0;
    let mut ray_y: f64 = 0.0;

    let mut xoff: f64 = 0.0;
    let mut yoff: f64 = 0.0;
    let mut max_dist = 0.0;
    let mut ray_a: f64 = player.a - (DT * FOV / 2.0);
    for r in 0..WIDTH {
        if ray_a < 0.0 {
            ray_a += 2.0 * PI;
        } else if ray_a > 2.0 * PI {
            ray_a -= 2.0 * PI;
        }
        //horizontal ray 

        let mut h_dist = 999999.0;
        let mut hrx = player.x;
        let mut hry = player.y;
        let mut dof = 0;
        let atan = -1.0 / ray_a.tan();

        if ray_a > PI {
            ray_y = (player.y / SCALE) as isize as f64 * SCALE;
            ray_x = (player.y - ray_y) * atan + player.x;
            yoff = -SCALE;
            xoff = -yoff * atan;
        } else if ray_a < PI {
            ray_y = ((player.y / SCALE) as isize as f64 * SCALE) + SCALE;
            ray_x = (player.y - ray_y) * atan + player.x;
            yoff = SCALE;
            xoff = -yoff * atan;
        } else if ray_a == 0.0 || ray_a == PI {
            ray_x = player.x;
            ray_y = player.y;
            dof = MAP_W;
        }
        while dof < MAP_W {
            let map_x = (ray_x / (SCALE)) as usize;
            let map_y = (ray_y / (SCALE)) as usize;
            if map_y > MAP_SIZE || map_x > MAP_SIZE {
                ray_x += xoff;
                ray_y += yoff;
                dof += 1;
            } else if (map_y * MAP_W) + map_x < MAP_SIZE && map[(map_y * MAP_W) + map_x] == 1 {
                hrx = ray_x;
                hry = ray_y;
                h_dist = dist(player.x, player.y, hrx, hry);
                break;
            } else {
                ray_x += xoff;
                ray_y += yoff;
                dof += 1;
            }
        }

        //vertical 
        let mut v_dist = 999999.0;
        let mut vrx = player.x;
        let mut vry = player.y;
        let mut dof = 0;
        let atan = -1.0 * ray_a.tan();
        // println!("tan {:?}",atan );
        if ray_a > PI / 2.0 && ray_a < 3.0 * PI / 2.0 {
            ray_x = (player.x / SCALE) as isize as f64 * SCALE;
            ray_y = (player.x - ray_x) * atan + player.y;
            xoff = -SCALE;
            yoff = -xoff * atan;
        } else if !(PI / 2.0..=2.0 * PI / 3.0).contains(&ray_a) {
            ray_x = ((player.x / SCALE) as isize as f64 * SCALE) + SCALE;
            ray_y = (player.x - ray_x) * atan + player.y;
            xoff = SCALE;
            yoff = -xoff * atan;
        } else if ray_a == 0.0 || ray_a == PI {
            ray_x = player.x;
            ray_y = player.y;
            dof = MAP_W;
        }
        // println!("v pos {} {}",ray_x/SCALE, ray_y/SCALE );

        while dof < MAP_W {
            let map_x = (ray_x / (SCALE)) as usize;
            let map_y = (ray_y / (SCALE)) as usize;
            if map_y > MAP_SIZE || map_x > MAP_SIZE {
                ray_x += xoff;
                ray_y += yoff;
                dof += 1;
            } else if (map_y * MAP_W) + map_x < MAP_SIZE && map[(map_y * MAP_W) + map_x] == 1 {
                vrx = ray_x;
                vry = ray_y;
                v_dist = dist(player.x, player.y, vrx, vry);
                break;
            } else {
                ray_x += xoff;
                ray_y += yoff;
                dof += 1;
            }
        }

        let mut fdist = v_dist;

        if v_dist >= h_dist {
            ray_x = hrx;
            ray_y = hry;
            fdist = h_dist;
        } else {
            ray_x = vrx;
            ray_y = vry;
        }
        if max_dist < fdist {
            max_dist = fdist;
        }

        let p_idx = interpolate(fdist*fdist, 0.0, 700.0*700.0, 06.0, 0.0) as usize;
        let c = PALLET[p_idx];
        let diff_a = ray_a - player.a;

        let mut line_h = (interpolate(fdist * diff_a.cos(), 0.0, 600.0, 60.0, 0.0)) as usize;
        if line_h > HEIGHT {
            line_h = HEIGHT - 1;
        }
        // println!("{:?}",fdist );
        let line_off: usize = (HEIGHT - line_h) / 2;
        // for y in line_off..line_off + line_h {
        for y in 0..HEIGHT {
            if y <= line_off {
                buffer[y][r] = ' ';
            } else if y > line_off && y < line_off + line_h {
                buffer[y][r] = c;
            } else {
                buffer[y][r] = '`';
            }
        }
        ray_a += DT;
    }
    (buffer, max_dist)
}

fn main() {
    //map
    let map = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1,
        1, 1, 1, 1,
    ];

    let mut player = Player {
        x: 100.0,
        y: 200.0,
        a: 0.0,
    };
    let _reverse = true;
    let _cchl = false;
    //main loop
    loop {
        let now = time::Instant::now();

        let (buffer, _max_dist) = doshit(&player, &map);
        // player movement
        // player.x+=10.0;
        if player.x < 300.0 {
            player.x += 10.0;
        } else if player.y <= 300.0 {
            player.y += 10.0;
        } else if player.a < PI {
            player.a += 2.0 * DT;
            if player.a > 2.0 * PI {
                player.a -= 2.0 * PI;
            } else if player.a < 0.0 {
                player.a += 2.0 * PI;
            }
        }

        // print!("{esc}c", esc = 27 as char);
        print!("{}[1;1H", 27 as char);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", buffer[y][x]);
            }
            println!();
        }
        // println!("x{} y{} a{} md{}",player.x,player.y,player.a,max_dist );
        // break;

        while now.elapsed().as_millis() < 200 { //adjust for changing fps
        }
    }
}
