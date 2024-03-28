use rand::Rng;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::util::{Color, Util};

pub trait Scatter: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Util::random_in_unit_sphere().normalize();
        if Util::near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Self {
        Metal { albedo: a, fuzz: f }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Util::reflect(&r_in.direction(), &rec.normal).normalize();
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Util::random_in_unit_sphere());

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    /*
        Now real glass has reflectivity that varies with angle - look at a window at a steep angle
        and it becomes mirror. There is a big ugly equation for that, but almost everybody uses the
        cheap and surprisingly accurate polynomial approximation by Christophe Schlick. This yields
        our full glass material.
    */
    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // use schlick's approxinmation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f32>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            Util::reflect(&unit_direction, &rec.normal)
        } else {
            Util::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
