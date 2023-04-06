use image::Rgb;
use image::RgbImage;
use ray_tracing::camera::Camera;
use ray_tracing::color::format_pixel_color;
use ray_tracing::hittable::*;
use ray_tracing::hittable_list::*;
use ray_tracing::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracing::ray::*;
use ray_tracing::sphere::*;
use ray_tracing::vec3::*;

use rand::prelude::*;
use threadpool::ThreadPool;

use std::sync::mpsc::channel;
use std::sync::Arc;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();

            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut sphere_material: Arc<dyn Material + Send + Sync> =
                    Arc::new(Metal::default());

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));

                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();

        if rec
            .material
            .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }

        return Color::default();
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let samples_per_pixel = 5;
    let max_depth = 2;

    // camera

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // world
    let world = random_scene();

    // Render
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);

    let mut img = RgbImage::new(image_width, image_height as u32);
    let (tx, rx) = channel();

    for j in 0..image_height {
        let world_ref = world.clone();
        let tx = tx.clone();
        pool.execute(move || {
            let mut rng = thread_rng();
            for i in 0..image_width {
                let mut pixel_color = Color::default();

                for _s in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                    let r = cam.get_ray(u, v);

                    pixel_color = pixel_color + ray_color(r, &world_ref, max_depth);
                }

                let rgb = format_pixel_color(pixel_color, samples_per_pixel);

                tx.send((i, (image_height - j - 1).abs(), rgb))
                    .expect("Could not send pixel data");
            }
        });
    }

    for _ in 0..(image_width as i32 * image_height) {
        let (x, y, (r, g, b)) = rx.recv().unwrap();
        let color = Rgb([r, g, b]);
        img.put_pixel(x as u32, y as u32, color);
    }

    let _ = img.save("output.png").unwrap();
}
