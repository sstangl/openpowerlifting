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
    const mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
    const mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;

    hideSearch();

    // If the navigation menu is open, reset its icon.
    if (mobileMenuToggler && mobileMenuToggler.classList) {
      mobileMenuToggler.classList.remove("active");
    }

    // Toggle the icon of the controls menu.
    if (mobileControlsBtn && mobileControlsBtn.classList) {
      mobileControlsBtn.classList.toggle("active");
    }

    // Hides the mobile menu when the user clicks on the filters menu.
    if (mobileMenu && mobileMenu.classList) {
      mobileMenu.classList.remove("open");
    }

    // Toggle the filters menu.
    if (filtersMobileMenu && filtersMobileMenu.classList) {
      filtersMobileMenu.classList.toggle("open");
    }
}

// Called when the hamburger menu is pressed.
function toggleMobileMenu(): void {
    const filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
    const mobileMenu = document.getElementById("mobile-menu-popup") as HTMLDivElement;
    const mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;
    const mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;

    hideSearch();

    // If the controls menu is open, reset its icon.
    if (mobileControlsBtn && mobileControlsBtn.classList) {
      mobileControlsBtn.classList.remove("active");
    }

    // Toggle the icon of the navigation menu.
    if (mobileMenuToggler && mobileMenuToggler.classList) {
      mobileMenuToggler.classList.toggle("active");
    }

    // Hide the filters menu when the user clicks on the main menu.
    if (filtersMobileMenu && filtersMobileMenu.classList) {
      filtersMobileMenu.classList.remove("open");
    }

    // Toggle the mobile menu.
    if (mobileMenu && mobileMenu.classList) {
      mobileMenu.classList.toggle("open");
    }
}

function hideMenus():void {
  const filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
  const mobileMenu = document.getElementById("mobile-menu-popup") as HTMLDivElement;
  const mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
  const mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;

  // If the navigation menu is open, reset its icon.
  if (mobileMenuToggler && mobileMenuToggler.classList) {
    mobileMenuToggler.classList.remove("active");
  }

  // If the controls menu is open, reset its icon.
  if (mobileControlsBtn && mobileControlsBtn.classList) {
    mobileControlsBtn.classList.remove("active");
  }

  // If the navigation menu is open, close it.
  if (mobileMenu && mobileMenu.classList) {
    mobileMenu.classList.remove("open");
  }

  // If the controls menu is open, close it.
  if (filtersMobileMenu && filtersMobileMenu.classList) {
    filtersMobileMenu.classList.remove("open");
  }
}

function hideSearch(): void {
  const searchContainer = document.getElementById("footerSearchContainer") as HTMLDivElement;
  const searchToggler = document.getElementById("footerSearchToggler") as HTMLButtonElement;

  if (searchContainer && searchContainer.classList) {
    searchContainer.classList.remove("open");
  }

  if (searchToggler && searchToggler.classList) {
    searchToggler.classList.remove("active");
  }
}

function toggleSearch(): void {
  const searchContainer = document.getElementById("footerSearchContainer") as HTMLDivElement;
  const searchToggler = document.getElementById("footerSearchToggler") as HTMLButtonElement;

  hideMenus();

  if (searchContainer && searchContainer.classList) {
    searchContainer.classList.toggle("open");
  }

  if (searchToggler && searchToggler.classList) {
    searchToggler.classList.toggle("active");
  }
}

function initMobileFooter(): void {
  const mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
  const mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;
  const mobileMenuLinks = document.getElementsByClassName("nav__link_mobile") as HTMLCollection;
  const searchField = document.getElementById("searchfield") as HTMLInputElement;
  const searchToggler = document.getElementById("footerSearchToggler") as HTMLButtonElement;

  if (searchToggler) {
     searchToggler.addEventListener("click", toggleSearch, false);
  }
  if (searchField) {
     searchField.addEventListener("focus", hideMenus, false);
  }
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
