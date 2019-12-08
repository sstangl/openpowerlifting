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

// Implementation of navigation and control event handlers for use on mobile pages.
// Mobile pages have a footer with pop-ups for controls, due to limited space.

// Called when the filters button is pressed. Not every page has one.
function toggleMobileFilters(): void {
    const mobileMenu = document.getElementById("mobile-menu-popup") as HTMLDivElement;
    const filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;

    // Hides the mobile menu when the user clicks on the filters menu.
    if (mobileMenu && mobileMenu.classList) {
      mobileMenu.classList.add("hide");
    }

    // Toggle the filters menu.
    if (filtersMobileMenu && filtersMobileMenu.classList) {
      filtersMobileMenu.classList.toggle("hide");
    }
}

// Called when the hamburger menu is pressed.
function toggleMobileMenu(): void {
    const filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
    const mobileMenu = document.getElementById("mobile-menu-popup") as HTMLDivElement;

    // Hide the filters menu when the user clicks on the main menu.
    if (filtersMobileMenu && filtersMobileMenu.classList) {
      filtersMobileMenu.classList.add("hide");
    }

    // Toggle the mobile menu.
    if (mobileMenu && mobileMenu.classList) {
      mobileMenu.classList.toggle("hide");
    }
}

function initMobileFooter(): void {
  const mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
  const mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;
  const mobileMenuLinks = document.getElementsByClassName("nav__link_mobile") as HTMLCollection;

  if (mobileControlsBtn) {
    mobileControlsBtn.addEventListener("click", toggleMobileFilters, false);
  }

  if (mobileMenuToggler) {
    mobileMenuToggler.addEventListener("click", toggleMobileMenu, false);
  }

  if (mobileMenuLinks && mobileMenuLinks.length > 0) {
    for (let i = 0; i < mobileMenuLinks.length; i++) {
      mobileMenuLinks[i].addEventListener("click", toggleMobileMenu, false);
    }
  }
}

// Determines whether the template is a mobile template, instead of a desktop template.
// Mobile templates all share the same navigational footer.
function isMobile(): boolean {
    return !!document.getElementById("mobile-footer");
}

export {
  initMobileFooter,
  isMobile
}
