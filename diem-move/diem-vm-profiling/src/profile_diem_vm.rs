// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{BUILD_PROFILE, PATH_CRATE_ROOT};
use anyhow::{bail, Result};
use diem_language_e2e_tests::data_store::GENESIS_CHANGE_SET_HEAD;
use once_cell::sync::Lazy;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const RUN_DIEM_P2P: &str = "run-diem-p2p";

static PATH_BIN_RUN_DIEM_P2P: Lazy<PathBuf> = Lazy::new(|| {
    PATH_CRATE_ROOT
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join(BUILD_PROFILE)
        .join(RUN_DIEM_P2P)
});

fn run_diem_p2p() -> Result<()> {
    println!("Profiling Diem VM...");

    let genesis_blob = bcs::to_bytes(GENESIS_CHANGE_SET_HEAD.write_set())?;

    let log_path = Path::join(&PATH_CRATE_ROOT, "p2p.log");
    let annotation_path = Path::join(&PATH_CRATE_ROOT, "p2p.txt");

    crate::valgrind::profile_with_valgrind(
        [&*PATH_BIN_RUN_DIEM_P2P],
        &genesis_blob,
        log_path,
        annotation_path,
    )?;

    Ok(())
}

fn build_binaries() -> Result<()> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--profile")
        .arg(BUILD_PROFILE)
        .arg("-p")
        .arg("diem-vm-profiling")
        .arg("--bin")
        .arg(RUN_DIEM_P2P)
        .status()?;

    if !status.success() {
        bail!("Failed to compile {}", RUN_DIEM_P2P);
    }

    Ok(())
}

pub fn run() -> Result<()> {
    build_binaries()?;
    run_diem_p2p()?;

    Ok(())
}
