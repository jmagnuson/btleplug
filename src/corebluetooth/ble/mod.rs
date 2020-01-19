// btleplug Source Code File
//
// Copyright 2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
//
// Some portions of this file are taken and/or modified from blurmac
// (https://github.com/servo/devices), using a BSD 3-Clause license under the
// following copyright:
//
// Copyright (c) 2017 Akos Kiss.
//
// Licensed under the BSD 3-Clause License
// <LICENSE.md or https://opensource.org/licenses/BSD-3-Clause>.
// This file may not be copied, modified, or distributed except
// according to those terms.

pub(crate) mod adapter;
pub(crate) mod delegate;
pub(crate) mod discovery_session;
pub(crate) mod device;
pub(crate) mod gatt_service;
pub(crate) mod gatt_characteristic;
pub(crate) mod gatt_descriptor;
pub(crate) mod framework;
pub(crate) mod utils;