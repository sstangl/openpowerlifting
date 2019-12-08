// vim: set ts=4 sts=4 sw=4 et:
//
// This file is part of OpenPowerlifting, an open archive of powerlifting data.
// Copyright (C) 2019 The OpenPowerlifting Project.
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

function changeLanguage(event): void {
    const newValue = event.target.value;

    const time = new Date();
    time.setFullYear(time.getFullYear()+3);
    const expiration = time.toUTCString();


    document.cookie="lang=" + newValue+ "; expires=" + expiration + "; path=/; ";
    var h = window.location.href;
    window.location.href = h.substring(0, h.indexOf("?"));
}

function changeUnits(event): void {
    const newValue = event.target.value;

    const time = new Date();
    time.setFullYear(time.getFullYear()+3);
    const expiration = time.toUTCString();

    document.cookie="units=" + newValue + "; expires=" + expiration + "; path=/; ";
    window.location.href = window.location.href;
}

function initLocaleEventListeners(): void {
    const weightunits = document.getElementById("weightunits") as HTMLSelectElement;
    if (weightunits) {
        weightunits.addEventListener("change", changeUnits);
    }

    const langselect = document.getElementById("langselect") as HTMLSelectElement;
    if (langselect) {
        langselect.addEventListener("change", changeLanguage);
    }
}

export {
  changeLanguage,
  changeUnits,
  initLocaleEventListeners
}
