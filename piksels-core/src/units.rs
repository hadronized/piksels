//! Units and binding points for indexed scarce resources, such as textures and uniform buffers.

use std::collections::HashMap;

use piksels_backend::{Backend, Unit};

#[derive(Debug, Eq, PartialEq)]
pub struct Units<B>
where
  B: Backend,
{
  next_unit: B::Unit,
  max_unit: B::Unit,
  idle_units: HashMap<B::Unit, B::ScarceIndex>,
}

impl<B> Units<B>
where
  B: Backend,
{
  pub fn new(max_unit: B::Unit) -> Self {
    Self {
      next_unit: Default::default(),
      max_unit,
      idle_units: HashMap::default(),
    }
  }

  /// Get a unit to bind to.
  pub fn get_unit(&mut self) -> Option<UnitBindingPoint<B>> {
    if self.next_unit < self.max_unit {
      // we still can use a fresh unit
      let unit = self.next_unit.clone();
      self.next_unit.next_unit();

      Some(UnitBindingPoint {
        unit,
        current_scarce_index: None,
      })
    } else {
      // we have exhausted the device units; try to reuse an idle one and if we cannot, then it’s an error
      self.reuse_unit()
    }
  }

  /// Try to reuse a binding. Return [`None`] if no binding is available, or a [`UnitBindingPoint`] mapping a unit
  /// with the currently bound scarce resource index otherwise.
  fn reuse_unit(&mut self) -> Option<UnitBindingPoint<B>> {
    let unit = self.idle_units.keys().next().cloned()?;
    let current_scarce_index = self.idle_units.remove(&unit)?;
    Some(UnitBindingPoint {
      unit,
      current_scarce_index: Some(current_scarce_index),
    })
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnitBindingPoint<B>
where
  B: Backend,
{
  /// Unit the binding point refers to.
  unit: B::Unit,

  /// Currently bound resource; [`None`] if no resource is bound to this unit.
  current_scarce_index: Option<B::ScarceIndex>,
}
