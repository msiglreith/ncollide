use na;
use shape::Segment;
use query::{PointQuery, PointProjection, RichPointQuery, PointNormalQuery, PointNormalProjection};
use math::{Point, Isometry};
use na::{normalize, Point2, Vector2};
use alga::general::Real;


impl<P: Point, M: Isometry<P>> PointQuery<P, M> for Segment<P> {
    #[inline]
    fn project_point(&self, m: &M, pt: &P, solid: bool) -> PointProjection<P> {
        let (projection, _) = self.project_point_with_extra_info(m, pt, solid);
        projection
    }

    // NOTE: the default implementation of `.distance_to_point(...)` will return the error that was
    // eaten by the `::approx_eq(...)` on `project_point(...)`.
}

impl<P: Point, M: Isometry<P>> RichPointQuery<P, M> for Segment<P> {
    type ExtraInfo = P::Real;

    #[inline]
    fn project_point_with_extra_info(&self, m: &M, pt: &P, _: bool)
        -> (PointProjection<P>, Self::ExtraInfo)
    {
        let ls_pt = m.inverse_transform_point(pt);
        let ab    = *self.b() - *self.a();
        let ap    = ls_pt - *self.a();
        let ab_ap = na::dot(&ab, &ap);
        let sqnab = na::norm_squared(&ab);

        let proj;
        let position_on_segment;

        if ab_ap <= na::zero() {
            // Voronoï region of vertex 'a'.
            position_on_segment = na::zero();
            proj = m.transform_point(self.a());
        }
        else if ab_ap >= sqnab {
            // Voronoï region of vertex 'b'.
            position_on_segment = na::one();
            proj = m.transform_point(self.b());
        }
        else {
            assert!(sqnab != na::zero());

            // Voronoï region of the segment interior.
            position_on_segment = ab_ap / sqnab;
            proj = m.transform_point(&(*self.a() + ab * position_on_segment));
        }

        // FIXME: is this acceptable?
        let inside = relative_eq!(proj, *pt);

        (PointProjection::new(inside, proj), position_on_segment)
    }
}

// TODO: reduce code duplication
impl<N, M> PointNormalQuery<Point2<N>, M> for Segment<Point2<N>>
    where N: Real,
          M: Isometry<Point2<N>> {
    #[inline]
    fn project_point_with_normal(&self, m: &M, pt: &Point2<N>, solid: bool) -> PointNormalProjection<Point2<N>> {
        let ls_pt = m.inverse_transform_point(pt);
        let ab    = *self.b() - *self.a();
        let ap    = ls_pt - *self.a();
        let bp    = ls_pt - *self.b();
        let ab_ap = na::dot(&ab, &ap);
        let sqnab = na::norm_squared(&ab);
        let cross = ab.x * ap.y - ab.x * ap.x;

        let proj;
        let normal;

        if ab_ap <= na::zero() {
            // Voronoï region of vertex 'a'.
            proj = m.transform_point(self.a());
            normal = normalize(&ap);
        }
        else if ab_ap >= sqnab {
            // Voronoï region of vertex 'b'.
            proj = m.transform_point(self.b());
            normal = normalize(&bp);
        }
        else {
            assert!(sqnab != na::zero());

            // Voronoï region of the segment interior.
            proj = m.transform_point(&(*self.a() + ab * (ab_ap / sqnab)));
            normal = normalize(&Vector2::new(ab.y, -ab.x));
        }

        // FIXME: is this acceptable?
        let inside = relative_eq!(&proj, pt);

        PointNormalProjection::new(inside, proj, normal)
    }
}
