// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0

//! # Verify File Check for non-existing Defaults File

use rust_kvs::{ErrorCode, InstanceId, Kvs, KvsApi, OpenNeedDefaults, OpenNeedKvs};

mod common;
use crate::common::TempDir;

/// Start with no KVS and check if the `need_defaults` flag is working
#[test]
fn kvs_check_needs_defaults() -> Result<(), ErrorCode> {
    let dir = TempDir::create()?;
    dir.set_current_dir()?;

    let kvs = Kvs::open(
        InstanceId::new(0),
        OpenNeedDefaults::Required,
        OpenNeedKvs::Optional,
    );

    assert_eq!(kvs.err(), Some(ErrorCode::KvsFileReadError));

    Ok(())
}
