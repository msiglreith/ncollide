use alga::general::Id;
use shape::{SupportMap, Reflection, MinkowskiSum};
use query::{Ray, RayCast};
use math::{Point, Isometry};


/// Time of impacts between two support-mapped shapes under translational movement.
pub fn support_map_against_support_map<P, M, G1: ?Sized, G2: ?Sized>(m1: &M, vel1: &P::Vector, g1: &G1,
                                                                     m2: &M, vel2: &P::Vector, g2: &G2)
                                                                     -> Option<P::Real>
    where P:  Point,
          M:  Isometry<P>,
          G1: SupportMap<P, M>,
          G2: SupportMap<P, M> {
    let vel = *vel1 - *vel2;
    let rg2 = Reflection::new(g2);
    let cso = MinkowskiSum::new(m1, g1, m2, &rg2);

    cso.toi_with_ray(&Id::new(), &Ray::new(P::origin(), -vel), true)
}

/// Time of impacts between two support-mapped shapes under translational movement.
pub fn support_map_against_support_map_with_normal<P, M, G1: ?Sized, G2: ?Sized>(m1: &M, vel1: &P::Vect, g1: &G1,
                                                                     m2: &M, vel2: &P::Vect, g2: &G2)
                                                                     -> Option<(<P::Vect as Vector>::Scalar, P::Vect)>
    where P:  Point,
          M:  Rotate<P::Vect> + Transform<P>,
          G1: SupportMap<P, M>,
          G2: SupportMap<P, M> {
    let vel = *vel1 - *vel2;
    let rg2 = Reflection::new(g2);
    let cso = MinkowskiSum::new(m1, g1, m2, &rg2);

    cso.toi_and_normal_with_ray(&Identity::new(), &Ray::new(na::origin(), -vel), true)
      .map(|inter| (inter.toi, inter.normal))
}