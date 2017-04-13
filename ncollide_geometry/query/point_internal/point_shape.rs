use math::{Point, Isometry};
use shape::Shape;
use query::{PointQuery, PointNormalQuery, PointProjection, PointNormalProjection};


impl<P, M> PointQuery<P, M> for Shape<P, M>
    where P: Point,
          M: Isometry<P> {
    #[inline]
    fn project_point(&self, m: &M, pt: &P, solid: bool) -> PointProjection<P> {
        self.as_point_query()
            .expect("No PointQuery implementation for the underlying shape.")
            .project_point(m, pt, solid)
    }

    #[inline]
    fn distance_to_point(&self, m: &M, pt: &P, solid: bool) -> P::Real {
        self.as_point_query()
            .expect("No PointQuery implementation for the underlying shape.")
            .distance_to_point(m, pt, solid)
    }

    #[inline]
    fn contains_point(&self, m: &M, pt: &P) -> bool {
        self.as_point_query()
            .expect("No PointQuery implementation for the underlying shape.")
            .contains_point(m, pt)
    }
}

impl<P, M> PointNormalQuery<P, M> for Shape<P, M>
    where P: Point,
          M: Isometry<P> + Translation<P::Vect> {
    #[inline]
    fn project_point_with_normal(&self, m: &M, pt: &P, solid: bool) -> PointNormalProjection<P> {
        self.as_point_normal_query()
            .expect("No PointNormalQuery implementation for the underlying shape.")
            .project_point_with_normal(m, pt, solid)
    }
}
