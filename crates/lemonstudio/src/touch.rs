// Copyright (C) 2026 ItsAlphaNeon
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Lemon Studio: https://github.com/ItsAlphaNeon/Lemon-Studio

//! Shared touch and stylus input types, and KDE's touch-input / on-screen-keyboard connection.

use std::time::Instant;
use winit::event::DeviceId;
use winit::window::WindowId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)] // Pan and Right are constructed by the feature branches that add those gestures.
pub(crate) enum TouchStage {
    Pending,
    Left,
    Pan,
    Right,
}

#[derive(Clone, Copy)]
pub(crate) struct TouchGesture {
    pub(crate) window: WindowId,
    pub(crate) device: DeviceId,
    pub(crate) finger: u64,
    pub(crate) from: (f64, f64),
    pub(crate) last: (f64, f64),
    /// Used by gestures with a hold deadline.
    #[allow(dead_code)]
    pub(crate) at: Instant,
    pub(crate) stage: TouchStage,
    /// Came in as a mouse event (an X11 stylus), rather than `WindowEvent::Touch`.
    pub(crate) mouse: bool,
}

impl TouchGesture {
    pub(crate) fn dragged(&self, at: (f64, f64)) -> bool {
        (at.0 - self.from.0).abs().max((at.1 - self.from.1).abs()) > f64::from(os_app::hold::SLOP)
    }
}

/// One session connection for KDE's touch-input and on-screen-keyboard interfaces.
#[cfg(target_os = "linux")]
#[derive(Default)]
pub(crate) struct Kde {
    connection: Option<zbus::blocking::Connection>,
}

#[cfg(target_os = "linux")]
impl Kde {
    pub(crate) fn proxy(&mut self) -> Option<zbus::blocking::Proxy<'_>> {
        if self.connection.is_none() {
            self.connection = zbus::blocking::Connection::session().ok();
        }
        zbus::blocking::Proxy::new(self.connection.as_ref()?, "org.kde.KWin", "/VirtualKeyboard", "org.kde.kwin.VirtualKeyboard").ok()
    }

    pub(crate) fn will_show_on_active(&mut self) -> bool {
        self.proxy().and_then(|p| p.call("willShowOnActive", &()).ok()).unwrap_or(false)
    }

    /// KWin's last input was touch or tablet, not a real mouse (NonMouseInput mode).
    pub(crate) fn stylus(&mut self) -> bool {
        let mode = self.proxy().and_then(|p| p.get_property("mode").ok()).unwrap_or(0);
        mode == 1 && self.will_show_on_active()
    }
}
