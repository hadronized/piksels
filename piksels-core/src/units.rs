//! Units and binding points for indexed scarce resources, such as textures and uniform buffers.

use std::collections::HashMap;

use piksels_backend::{error::Error, Backend, Unit};

#[derive(Debug, Eq, PartialEq)]
pub struct Units<B, U>
where
  B: Backend,
  U: Unit,
{
  next_unit: U,
  max_units: U,
  idle_units: HashMap<U, B::ScarceIndex>,
}

impl<B, U> Units<B, U>
where
  B: Backend,
  U: Unit,
{
  pub fn new(max_unit: U) -> Self {
    Self {
      next_unit: Default::default(),
      max_units: max_unit,
      idle_units: HashMap::default(),
    }
  }

  /// Get a unit to bind to.
  pub fn get_unit(&mut self) -> Result<UnitBindingPoint<B, U>, B::Err> {
    if self.next_unit < self.max_units {
      // we still can use a fresh unit
      let unit = self.next_unit.clone();
      self.next_unit.next_unit();

      Ok(UnitBindingPoint {
        unit,
        current_scarce_index: None,
      })
    } else {
      // we have exhausted the device units; try to reuse an idle one and if we cannot, then it’s an error
      self.reuse_unit().ok_or(Error::NoMoreUnits.into())
    }
  }

  /// Try to reuse a binding. Return [`None`] if no binding is available, or a [`UnitBindingPoint`] mapping a unit
  /// with the currently bound scarce resource index otherwise.
  fn reuse_unit(&mut self) -> Option<UnitBindingPoint<B, U>> {
    let unit = self.idle_units.keys().next().cloned()?;
    let current_scarce_index = self.idle_units.remove(&unit)?;

    Some(UnitBindingPoint {
      unit,
      current_scarce_index: Some(current_scarce_index),
    })
  }

  /// Mark a unit as idle.
  pub fn idle(&mut self, unit: U, scarce_index: B::ScarceIndex) {
    self.idle_units.insert(unit, scarce_index);
  }

  /// Mark a unit as non-idle (in-use).
  pub fn in_use(&mut self, unit: U) {
    self.idle_units.remove(&unit);
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnitBindingPoint<B, U>
where
  B: Backend,
  U: Unit,
{
  /// Unit the binding point refers to.
  pub(crate) unit: U,

  /// Currently bound resource; [`None`] if no resource is bound to this unit.
  pub(crate) current_scarce_index: Option<B::ScarceIndex>,
}
