//! Units and binding points for indexed scarce resources, such as textures and uniform buffers.

use std::collections::HashMap;

use piksels_backend::{error::Error, Backend, Unit};

#[derive(Debug, Eq, PartialEq)]
pub struct Units<B>
where
  B: Backend,
{
  next_unit: B::Unit,
  max_units: B::Unit,
  idle_units: HashMap<B::Unit, B::ScarceIndex>,
}

impl<B> Units<B>
where
  B: Backend,
{
  pub fn new(max_unit: B::Unit) -> Self {
    Self {
      next_unit: Default::default(),
      max_units: max_unit,
      idle_units: HashMap::default(),
    }
  }

  /// Get a unit to bind to.
  pub fn get_unit(&mut self) -> Result<UnitBindingPoint<B>, B::Err> {
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
  fn reuse_unit(&mut self) -> Option<UnitBindingPoint<B>> {
    let unit = self.idle_units.keys().next().cloned()?;
    let current_scarce_index = self.idle_units.remove(&unit)?;

    Some(UnitBindingPoint {
      unit,
      current_scarce_index: Some(current_scarce_index),
    })
  }

  /// Mark a unit as idle.
  pub fn idle(&mut self, unit: B::Unit, scarce_index: B::ScarceIndex) {
    self.idle_units.insert(unit, scarce_index);
  }

  /// Mark a unit as non-idle (in-use).
  pub fn in_use(&mut self, unit: B::Unit) {
    self.idle_units.remove(&unit);
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnitBindingPoint<B>
where
  B: Backend,
{
  /// Unit the binding point refers to.
  pub(crate) unit: B::Unit,

  /// Currently bound resource; [`None`] if no resource is bound to this unit.
  pub(crate) current_scarce_index: Option<B::ScarceIndex>,
}
