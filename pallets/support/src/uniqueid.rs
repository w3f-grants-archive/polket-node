// This file is part of Polket.
// Copyright (C) 2021-2022 Polket.
// SPDX-License-Identifier: GPL-3.0-or-later

pub trait UniqueIdGenerator {
	type ObjectId;

	/// generate new object id by parentId, Return the current ID, and increment the current ID
	fn generate_object_id(parentId: Self::ObjectId) -> Result<Self::ObjectId, sp_runtime::DispatchError>;
}
