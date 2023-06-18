//                                                   ~-.
//                                                   ,,,;            ~-.~-.~-
//                                               (.../           ~-.~-.~-.~-.~-.
//                                           < } O~`, ,        ~-.~-.~-.~-.~-.~-.
//                                               (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
//                                                   ;    T     ~-.~-.~-.~-.~-.~-.~-.
//                                                 ;   {_.~-.~-.~-.~-.~-.~-.~
//                                               ;:  .-~`    ~-.~-.~-.~-.~-.
//                                               ;.: :'    ._   ~-.~-.~-.~-.~-
//                                               ;::`-.    '-._  ~-.~-.~-.~-
//                                               ;::. `-.    '-,~-.~-.~-.
//                                                   ';::::.`''-.-'
//                                                   ';::;;:,:'
//                                                       '||T
//                                                     __   _
//    

use crate::*;
use frame_support::fail;

// IMPLS
impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
    type CurrencyId = AssetId;
    type Balance = T::Balance;

    fn minimum_balance(_currency_id: Self::CurrencyId) -> Self::Balance {
        log::trace!(
            "minimum_balance",
        );
        Default::default()
    }

    fn total_issuance(_currency_id: Self::CurrencyId) -> Self::Balance {
        log::trace!(
            "total_issuance",
        );
        Default::default()
    }

    fn total_balance(_currency_id: Self::CurrencyId, _who: &T::AccountId) -> Self::Balance {
        log::trace!(
            "total_balance",
        );
        Default::default()
    }

    fn free_balance(_currency_id: Self::CurrencyId, _who: &T::AccountId) -> Self::Balance {
        log::trace!(
            "free_balance",
        );
        Default::default()
    }

    fn ensure_can_withdraw(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            "ensure_can_withdraw",
        );
        Ok(())
    }

    fn transfer(
        _currency_id: Self::CurrencyId,
        _from: &T::AccountId,
        _to: &T::AccountId,
        _amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            "transfer",
        );
        fail!(Error::<T>::MethodNotAvailible)
    }

    /// THIS
    fn deposit(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::debug!(
            "deposit || currency_id: {:?} || who: {:?} || amount: {:?} ||",
            currency_id, 
            who,
            amount,
        );
        Self::deposit_event(Event::<T>::Deposit(currency_id, who.clone(), amount));
        Ok(())
    }

    fn withdraw(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            "withdraw",
        );
        Ok(())
    }

    fn can_slash(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _value: Self::Balance,
    ) -> bool {
        log::trace!(
            "can_slash",
        );
        true
    }

    fn slash(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _amount: Self::Balance,
    ) -> Self::Balance {
        Default::default()
    }
}

// IMPLS for p_runtime::traits::Convert trait to allow this pallet be used as Converter in XCM localasset transactor:
impl<T: Config> sp_runtime::traits::Convert<AssetId, Option<MultiLocation>> for Pallet<T> {
    fn convert(id: AssetId) -> Option<MultiLocation> {
        let maybe_multilocation = Pallet::<T>::get_multilocation_from_asset_id(id);
        if maybe_multilocation.is_none() {
            Self::deposit_event(Event::<T>::AssetIdMappingError(id));
        }
        maybe_multilocation
    }
}

impl<T: Config> sp_runtime::traits::Convert<MultiLocation, Option<AssetId>> for Pallet<T> {
    fn convert(multilocation: MultiLocation) -> Option<AssetId> {
        let maybe_asset_id = Pallet::<T>::get_asset_id_from_multilocation(multilocation.clone());
        if maybe_asset_id.is_none() {
            Self::deposit_event(Event::<T>::MultilocationMappingError(multilocation));
        }
        maybe_asset_id
    }
}

impl<T: Config> sp_runtime::traits::Convert<MultiAsset, Option<AssetId>> for Pallet<T> {
    fn convert(ma: MultiAsset) -> Option<AssetId> {
        if let MultiAsset { fun: Fungible(_), id: Concrete(ml) } = ma {
            Self::convert(ml)
        } else {
            Self::deposit_event(Event::<T>::MultiAssetMappingError(ma));
            Option::None
        }
    }
}
