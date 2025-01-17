// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import "../libraries/Json.sol";

contract JsonTest {
    function callGetString(
        string memory json,
        string memory pointer
    ) public returns (bool, string memory) {
        return Json.getString(json, pointer);
    }

    function callGetI64(
        string memory json,
        string memory pointer
    ) public returns (bool, int64) {
        return Json.getI64(json, pointer);
    }

    function callGetBool(
        string memory json,
        string memory pointer
    ) public returns (bool, bool) {
        return Json.getBool(json, pointer);
    }

    function callGetArrayLen(
        string memory json,
        string memory pointer
    ) public returns (bool, int64) {
        return Json.getArrayLen(json, pointer);
    }
}
