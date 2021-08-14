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

// Mobile base.tera files supply an IS_MOBILE global.
declare const IS_MOBILE: boolean | undefined;

// Called when the filters button is pressed. Not every page has one.
let mobileMenu: HTMLDivElement;
let controlsMenu: HTMLDivElement;
let mobileControlsBtn: HTMLElement;
let mobileMenuToggler: HTMLElement;

function showMobileMenu(): void {
  if (mobileMenu && mobileMenu.classList) {
    mobileMenu.classList.add("open");
  }
}
function hideMobileMenu(): void {
  if (mobileMenu && mobileMenu.classList) {
    mobileMenu.classList.remove("open");
  }
}

function showMobileControls(): void {
  if (controlsMenu && controlsMenu.classList) {
    controlsMenu.classList.add("open");
  }
}
function hideMobileControls(): void {
  if (controlsMenu && controlsMenu.classList) {
    controlsMenu.classList.remove("open");
  }
}

function removeClassFromBody(): void {
  const body = document.body;
  body.classList.remove("menu-open");
}
// this class will disable scrolling of the content when mobile menu is open
function addClassToBody(): void {
  const body = document.body;
  body.classList.add("menu-open");
}

function toggleMobileFilters(): void {
    // If the navigation menu is open, reset its icon.
    if (mobileMenuToggler && mobileMenuToggler.classList) {
      mobileMenuToggler.classList.remove("active");
    }

    // Toggle the icon of the controls menu.
    if (mobileControlsBtn && mobileControlsBtn.classList) {
      mobileControlsBtn.classList.toggle("active");
    }

    hideMobileMenu();

    if (controlsMenu.classList.contains('open')) {
      hideMobileControls();
      removeClassFromBody();
    } else {
      hideSearch();
      showMobileControls();
      addClassToBody();
    }
}

// Called when the hamburger menu is pressed.
function toggleMobileMenu(): void {
    // If the controls menu is open, reset its icon.
    if (mobileControlsBtn && mobileControlsBtn.classList) {
      mobileControlsBtn.classList.remove("active");
    }

    // Toggle the icon of the navigation menu.
    if (mobileMenuToggler && mobileMenuToggler.classList) {
      mobileMenuToggler.classList.toggle("active");
    }

    hideMobileControls();

    if (mobileMenu.classList.contains('open')) {
      hideMobileMenu();
      removeClassFromBody();
    } else {
      hideSearch();
      showMobileMenu();
      addClassToBody();
    }

}

function hideMenus():void {
  // If the navigation menu is open, reset its icon.
  if (mobileMenuToggler && mobileMenuToggler.classList) {
    mobileMenuToggler.classList.remove("active");
  }

  // If the controls menu is open, reset its icon.
  if (mobileControlsBtn && mobileControlsBtn.classList) {
    mobileControlsBtn.classList.remove("active");
  }

  hideMobileMenu();
  hideMobileControls()
  removeClassFromBody();
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
  controlsMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
  mobileMenu = document.getElementById("mobile-menu-popup") as HTMLDivElement;
  mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
  mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;

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
function isMobile(): boolean {
    if (typeof IS_MOBILE === "boolean") {
        return IS_MOBILE === true;
    }
    return false;
}

export {
  initMobileFooter,
  isMobile
}
