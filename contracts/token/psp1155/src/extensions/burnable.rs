/// Extension of [`PSP1155`] that allows token holders to destroy their tokens
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::trait_definition]
pub trait PSP1155Burnable: PSP1155 {
    /// Destroys `amount` tokens of token type `id` from the user
    ///
    /// See [`PSP1155::_burn`].
    #[ink(message)]
    fn burn(&mut self, id: Id, amount: Balance) {
        self._burn(Self::env().caller(), id, amount);
    }

    /// Destroys `amount` tokens of token type `id` from `from`
    ///
    /// See [`PSP1155::_burn`].
    #[ink(message)]
    fn burn_from(&mut self, from: AccountId, id: Id, amount: Balance) {
        assert!(
            self.is_approved_for_all(from, Self::env().caller()),
            "{}",
            PSP1155Error::ApproveRequired.as_ref()
        );

        self._burn(from, id, amount);
    }

    /// Destroys `ids_amounts[i].1` of token type `ids_amounts[i].0` from the user
    ///
    /// See [`PSP1155::_burn`].
    #[ink(message)]
    fn burn_batch(&mut self, ids_amounts: Vec<(Id, Balance)>) {
        self._burn_batch(Self::env().caller(), ids_amounts);
    }

    /// Destroys `ids_amounts[i].1` of token type `ids_amounts[i].0` from `from` 
    ///
    /// See [`PSP1155::_burn`].
    #[ink(message)]
    fn burn_batch_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) {
        assert!(
            self.is_approved_for_all(from, Self::env().caller()),
            "{}",
            PSP1155Error::ApproveRequired.as_ref()
        );

        self._burn_batch(from, ids_amounts);
    }
}
