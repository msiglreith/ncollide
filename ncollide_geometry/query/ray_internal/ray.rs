//! Traits and structure needed to cast rays.

use na::Point2;
use math::{Point, Vector};

/// A Ray.
#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
pub struct Ray<P: Point> {
    /// Starting point of the ray.
    pub origin: P,
    /// Direction of the ray.
    pub dir:  P::Vect
}

impl<P: Point> Ray<P> {
    /// Creates a new ray starting from `origin` and with the direction `dir`. `dir` must be
    /// normalized.
    pub fn new(origin: P, dir: P::Vect) -> Ray<P> {
        Ray {
            origin: origin,
            dir:  dir
        }
    }
}

/// Structure containing the result of a successful ray cast.
pub struct RayIntersection<V: Vector> {
    /// The time of impact of the ray with the object.  The exact contact point can be computed
    /// with: `origin + dir * toi` where `origin` is the origin of the ray; `dir` is its direction and
    /// `toi` is the value of this field.
    pub toi:    V::Scalar,

    /// The normal at the intersection point.
    ///
    /// If the `toi` is exactly zero, the normal might not be reliable.
    pub normal: V,

    /// The textures coordinates at the intersection point.  This is an `Option` because some shape
    /// do not support texture coordinates.
    pub uvs:    Option<Point2<V::Scalar>>
}

impl<V: Vector> RayIntersection<V> {
    #[inline]
    /// Creates a new `RayIntersection`.
    pub fn new_with_uvs(toi: V::Scalar, normal: V, uvs: Option<Point2<V::Scalar>>) -> RayIntersection<V> {
        RayIntersection {
            toi:    toi,
            normal: normal,
            uvs:    uvs
        }
    }

    #[inline]
    /// Creates a new `RayIntersection`.
    pub fn new(toi: V::Scalar, normal: V) -> RayIntersection<V> {
        RayIntersection {
            toi:    toi,
            normal: normal,
            uvs:    None
        }
    }
}

/// Traits of objects which can be transformed and tested for intersection with a ray.
pub trait RayCast<P: Point, M> {
    /// Computes the time of impact between this transform shape and a ray.
    fn toi_with_ray(&self, m: &M, ray: &Ray<P>, solid: bool) -> Option<<P::Vect as Vector>::Scalar> {
        self.toi_and_normal_with_ray(m, ray, solid).map(|inter| inter.toi)
    }

    /// Computes the time of impact, and normal between this transformed shape and a ray.
    #[inline]
    fn toi_and_normal_with_ray(&self, m: &M, ray: &Ray<P>, solid: bool) -> Option<RayIntersection<P::Vect>>;

    /// Computes time of impact, normal, and texture coordinates (uv) between this transformed
    /// shape and a ray.
    #[inline]
    fn toi_and_normal_and_uv_with_ray(&self, m: &M, ray: &Ray<P>, solid: bool) -> Option<RayIntersection<P::Vect>> {
        self.toi_and_normal_with_ray(m, ray, solid)
    }

    /// Tests whether a ray intersects this transformed shape.
    #[inline]
    fn intersects_ray(&self, m: &M, ray: &Ray<P>) -> bool {
        self.toi_with_ray(m, ray, true).is_some()
    }
}
