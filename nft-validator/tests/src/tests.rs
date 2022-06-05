use super::*;
use blake2b_ref::Blake2bBuilder;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use rand::{thread_rng, Rng};

pub fn random_32bytes() -> Bytes {
    let mut rng = thread_rng();
    let mut buf = vec![0u8; 32];
    rng.fill(&mut buf[..]);
    Bytes::from(buf)
}

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_nft_transfer() {
    // deploy contract
    let mut context = Context::default();
    let nft_bin: Bytes = Loader::default().load_binary("nft-validator");
    let nft_out_point = context.deploy_cell(nft_bin);
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();
    let lock_script2 = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let governance_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let governance_script_hash = governance_script.calc_script_hash();
    let nft_type_script = context
        .build_script(&nft_out_point, governance_script_hash.raw_data())
        .expect("script");
    let nft_script_dep = CellDep::new_builder()
        .out_point(nft_out_point.clone())
        .build();

    // prepare cells
    let nft_id = random_32bytes();
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(
                ScriptOpt::new_builder()
                    .set(Some(nft_type_script.clone()))
                    .build(),
            )
            .build(),
        nft_id.clone(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![CellOutput::new_builder()
        .capacity(999u64.pack())
        .lock(lock_script2.clone())
        .type_(
            ScriptOpt::new_builder()
                .set(Some(nft_type_script.clone()))
                .build(),
        )
        .build()];

    let outputs_data = vec![nft_id];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(nft_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

#[test]
fn test_nft_generation() {
    // deploy contract
    let mut context = Context::default();
    let nft_bin: Bytes = Loader::default().load_binary("nft-validator");
    let nft_out_point = context.deploy_cell(nft_bin);
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();
    let governance_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let governance_script_hash = governance_script.calc_script_hash();
    let nft_type_script = context
        .build_script(&nft_out_point, governance_script_hash.raw_data())
        .expect("script");
    let nft_script_dep = CellDep::new_builder()
        .out_point(nft_out_point.clone())
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(10000u64.pack())
            .lock(governance_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(input.as_slice());
    blake2b.update(&1u64.to_le_bytes());
    let mut hash = vec![0u8; 32];
    blake2b.finalize(&mut hash[..]);
    let nft_id = Bytes::from(hash);

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(9500u64.pack())
            .lock(governance_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(499u64.pack())
            .lock(lock_script.clone())
            .type_(
                ScriptOpt::new_builder()
                    .set(Some(nft_type_script.clone()))
                    .build(),
            )
            .build(),
    ];

    let outputs_data = vec![Bytes::new(), nft_id];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(nft_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

#[test]
fn test_nft_invalid_governance() {
    // deploy contract
    let mut context = Context::default();
    let nft_bin: Bytes = Loader::default().load_binary("nft-validator");
    let nft_out_point = context.deploy_cell(nft_bin);
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();
    let nft_type_script = context
        .build_script(&nft_out_point, random_32bytes())
        .expect("script");
    let nft_script_dep = CellDep::new_builder()
        .out_point(nft_out_point.clone())
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(10000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(input.as_slice());
    blake2b.update(&1u64.to_le_bytes());
    let mut hash = vec![0u8; 32];
    blake2b.finalize(&mut hash[..]);
    let nft_id = Bytes::from(hash);

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(9500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(499u64.pack())
            .lock(lock_script.clone())
            .type_(
                ScriptOpt::new_builder()
                    .set(Some(nft_type_script.clone()))
                    .build(),
            )
            .build(),
    ];

    let outputs_data = vec![Bytes::new(), nft_id];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(nft_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let error = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_eq!("Script(ValidationFailure(6))", error.to_string());
}

#[test]
fn test_nft_invalid_nft_data() {
    // deploy contract
    let mut context = Context::default();
    let nft_bin: Bytes = Loader::default().load_binary("nft-validator");
    let nft_out_point = context.deploy_cell(nft_bin);
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();
    let governance_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let governance_script_hash = governance_script.calc_script_hash();
    let nft_type_script = context
        .build_script(&nft_out_point, governance_script_hash.raw_data())
        .expect("script");
    let nft_script_dep = CellDep::new_builder()
        .out_point(nft_out_point.clone())
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(10000u64.pack())
            .lock(governance_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(input.as_slice());
    blake2b.update(&1u64.to_le_bytes());
    let mut hash = vec![0u8; 32];
    blake2b.finalize(&mut hash[..]);
    let nft_id = Bytes::from(hash);

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(9500u64.pack())
            .lock(governance_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(499u64.pack())
            .lock(lock_script.clone())
            .type_(
                ScriptOpt::new_builder()
                    .set(Some(nft_type_script.clone()))
                    .build(),
            )
            .build(),
    ];

    let outputs_data = vec![Bytes::new(), nft_id.slice(0..16)];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(nft_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let error = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_eq!("Script(ValidationFailure(4))", error.to_string());
}

#[test]
fn test_nft_invalid_nft_hash() {
    // deploy contract
    let mut context = Context::default();
    let nft_bin: Bytes = Loader::default().load_binary("nft-validator");
    let nft_out_point = context.deploy_cell(nft_bin);
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();
    let governance_script = context
        .build_script(&always_success_out_point, random_32bytes())
        .expect("lock script");
    let governance_script_hash = governance_script.calc_script_hash();
    let nft_type_script = context
        .build_script(&nft_out_point, governance_script_hash.raw_data())
        .expect("script");
    let nft_script_dep = CellDep::new_builder()
        .out_point(nft_out_point.clone())
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(10000u64.pack())
            .lock(governance_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(input.as_slice());
    blake2b.update(&1u64.to_le_bytes());
    let mut hash = vec![0u8; 32];
    blake2b.finalize(&mut hash[..]);
    hash[0] += 1;
    let nft_id = Bytes::from(hash);

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(9500u64.pack())
            .lock(governance_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(499u64.pack())
            .lock(lock_script.clone())
            .type_(
                ScriptOpt::new_builder()
                    .set(Some(nft_type_script.clone()))
                    .build(),
            )
            .build(),
    ];

    let outputs_data = vec![Bytes::new(), nft_id];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(nft_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let error = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_eq!("Script(ValidationFailure(7))", error.to_string());
}
