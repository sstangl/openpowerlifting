// vim: set ts=4 sts=4 sw=4 et:
//
// This file is part of OpenPowerlifting, an open archive of powerlifting data.
// Copyright (C) 2021 The OpenPowerlifting Project.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Distribution-specific logic (OpenPowerlifting VS OpenIPF, etc).

// Flavor of website.
enum Distribution {
    OpenPowerlifting = "OPENPOWERLIFTING",
    OpenIpf = "OPENIPF",
}

// The distribution is set as a const in the base.tera file of each distribution.
declare const DISTRIBUTION: Distribution;

// Returns whether the current distribution for this template is OpenIPF.
function isOpenIpf() {
    return DISTRIBUTION === Distribution.OpenIpf;
}

export {
    isOpenIpf
}
