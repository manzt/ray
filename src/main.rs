mod ray;
mod vec3;

fn ray_color(ray: ray::Ray) -> vec3::Color {
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    let white = vec3::Color::new(1.0, 1.0, 1.0);
    let blue = vec3::Color::new(0.5, 0.7, 1.0);
    (1.0 - a) * white + a * blue
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = vec3::Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down
    // the vertical viewport edges
    let viewport_u = vec3::Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta
    // vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - vec3::Vector3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixiel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixiel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_diection = pixel_center - camera_center;
            let ray = ray::Ray::new(&camera_center, &ray_diection);
            {
                let (ir, ig, ib) = ray_color(ray).rgb();
                println!("{ir} {ig} {ib}");
            }
        }
    }
    eprint!("\rDone.                 \n");
}
