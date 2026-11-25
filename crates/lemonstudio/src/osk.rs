// Copyright (C) 2026 ItsAlphaNeon
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Lemon Studio: https://github.com/ItsAlphaNeon/Lemon-Studio

//! KDE's on-screen keyboard, driven through the shared KDE connection.

use crate::Shell;

/// Asked for only while a text field has the keyboard and Plasma would show it for the last input (touch or stylus by
/// default). X11 has no text-input focus protocol; KWin turns the keyboard's text into keys.
#[derive(Default)]
pub(crate) struct KdeOsk {
    pub(crate) text: bool,
    pub(crate) activated: bool,
}

impl Shell {
    pub(crate) fn osk_sync(&mut self, text: bool) {
        if self.osk.text == text {
            return;
        }
        self.osk.text = text;
        if !text {
            self.osk_hide();
            return;
        }
        let show = self.kde.will_show_on_active();
        if show && self.kde.proxy().and_then(|p| p.call::<_, _, ()>("forceActivate", &()).ok()).is_some() {
            self.osk.activated = true;
        }
    }

    pub(crate) fn osk_hide(&mut self) {
        self.osk.text = false;
        if !std::mem::take(&mut self.osk.activated) {
            return;
        }
        if let Some(p) = self.kde.proxy() {
            let _ = p.set_property("active", false);
        }
    }
}
