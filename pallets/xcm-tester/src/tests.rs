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

use crate::{mock::*, Error};
// use bridge_types::{types::AssetKind, H256};
use frame_support::{assert_noop, assert_ok};
use xcm::{
    opaque::latest::{
        Junction::{GeneralKey, Parachain},
        Junctions::X2,
    },
    v3::MultiLocation,
};

#[test]
fn it_works_register_change_delete() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let new_asset_id = [2; 32].into();
        let multilocation = MultiLocation::parent();
        let new_multilocation = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };

        // Create:
        assert_ok!(Transactor::register_mapping(asset_id, multilocation.clone()));
        assert_eq!(
            Transactor::get_multilocation_from_asset_id::<H256>(asset_id.into())
                .expect("it_works_register_change_delete, Create: multilocation is None"),
            multilocation.clone()
        );
        assert_eq!(
            Transactor::get_asset_id_from_multilocation(multilocation.clone())
                .expect("it_works_register_change_delete, Create: asset id is None"),
            asset_id
        );

        // Change Asset's Multilocation:
        assert_ok!(Transactor::change_asset_mapping(asset_id, new_multilocation.clone()));
        assert_eq!(
			Transactor::get_multilocation_from_asset_id(asset_id)
				.expect("it_works_register_change_delete, Change Asset's Multilocation: new_multilocation is None"),
			new_multilocation.clone()
		);
        assert_eq!(
            Transactor::get_asset_id_from_multilocation(new_multilocation.clone()).expect(
                "it_works_register_change_delete, Change Asset's Multilocation: asset_id is None"
            ),
            asset_id
        );
        assert_eq!(Transactor::get_asset_id_from_multilocation(multilocation.clone()), None);

        // Change Multilocation's Asset
        assert_ok!(Transactor::change_multilocation_mapping(new_multilocation.clone(), new_asset_id,));
        assert_eq!(
			Transactor::get_multilocation_from_asset_id(new_asset_id)
				.expect("it_works_register_change_delete, Change Multilocation's Asset: new_multilocation is None"),
			new_multilocation.clone()
		);
        assert_eq!(
            Transactor::get_asset_id_from_multilocation(new_multilocation.clone()).expect(
                "it_works_register_change_delete, Change Multilocation's Asset: asset_id is None"
            ),
            new_asset_id
        );
        assert_eq!(Transactor::get_multilocation_from_asset_id(asset_id), None);

        // Delete:
        assert_ok!(Transactor::delete_mapping(new_asset_id));
        assert_eq!(Transactor::get_multilocation_from_asset_id(new_asset_id), None);
        assert_eq!(Transactor::get_asset_id_from_multilocation(new_multilocation), None);
    });
}

#[test]
fn it_fails_create_existing_multilocation_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let multilocation = MultiLocation::parent();
        let new_multilocation = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };

        assert_ok!(Transactor::register_mapping(asset_id, multilocation.clone()));

        assert_noop!(
            Transactor::register_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
        assert_noop!(
            Transactor::register_mapping(asset_id, new_multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
    });
}

#[test]
fn it_fails_create_existing_asset_id_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let new_asset_id = [2; 32].into();
        let multilocation = MultiLocation::parent();

        assert_ok!(Transactor::register_mapping(asset_id, multilocation.clone()));

        assert_noop!(
            Transactor::register_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
        assert_noop!(
            Transactor::register_mapping(new_asset_id, multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
    });
}

#[test]
fn it_fails_change_asset_non_existing_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let new_asset_id = [2; 32].into();
        let multilocation = MultiLocation::parent();

        assert_noop!(
            Transactor::change_asset_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingNotExist
        );

        assert_ok!(Transactor::register_mapping(new_asset_id, multilocation.clone()));
        assert_noop!(
            Transactor::change_asset_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingNotExist
        );
        assert_eq!(
            Transactor::get_asset_id_from_multilocation(multilocation.clone())
                .expect("it_fails_change_asset_non_existing_mapping: asset id is None"),
            new_asset_id
        );
    });
}

#[test]
fn it_fails_change_multilocation_non_existing_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let multilocation = MultiLocation::parent();
        let new_multilocation = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };

        assert_noop!(
            Transactor::change_asset_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingNotExist
        );

        assert_ok!(Transactor::register_mapping(asset_id, new_multilocation.clone()));
        assert_noop!(
            Transactor::change_multilocation_mapping(multilocation.clone(), asset_id),
            Error::<Test>::MappingNotExist
        );
        assert_eq!(
            Transactor::get_multilocation_from_asset_id(asset_id)
                .expect("it_fails_change_multilocation_non_existing_mapping: asset id is None"),
            new_multilocation
        );
    });
}

#[test]
fn it_fails_delete_mapping_non_existing_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        assert_noop!(Transactor::delete_mapping(asset_id), Error::<Test>::MappingNotExist);
    });
}

#[test]
fn it_works_register_asset() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let multiasset = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };
        assert_ok!(Transactor::register_asset(
            RuntimeOrigin::root(),
            asset_id,
            multiasset.clone().into(),
            AssetKind::Sidechain,
        ));
        assert_eq!(
            Transactor::get_multilocation_from_asset_id::<H256>(asset_id.into())
                .expect("it_works_register_asset, Create: multilocation is None"),
            multiasset.clone()
        );
        assert_eq!(
            Transactor::get_asset_id_from_multilocation(multiasset.clone())
                .expect("it_works_register_asset, Create: asset id is None"),
            asset_id
        );
        let new_asset_id = [2; 32].into();
        assert_noop!(
            Transactor::register_asset(
                RuntimeOrigin::root(),
                new_asset_id,
                multiasset.clone().into(),
                AssetKind::Sidechain,
            ),
            Error::<Test>::MappingAlreadyExists
        );
        assert_eq!(
            Transactor::get_multilocation_from_asset_id::<H256>(asset_id.into())
                .expect("it_works_register_asset, Create: multilocation is None"),
            multiasset.clone()
        );
        assert_eq!(
            Transactor::get_asset_id_from_multilocation(multiasset)
                .expect("it_works_register_asset, Create: asset id is None"),
            asset_id
        );
    });
}
