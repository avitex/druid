// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! GTK Monitors and Screen information.

use crate::screen::Monitor;
use gtk::gdk::{Display, DisplayManager, Rectangle};
use kurbo::{Point, Rect, Size};

fn translate_gdk_rectangle(r: Rectangle) -> Rect {
    Rect::from_origin_size(
        Point::new(r.x() as f64, r.y() as f64),
        Size::new(r.width() as f64, r.height() as f64),
    )
}

fn translate_gdk_monitor(mon: gtk::gdk::Monitor) -> Monitor {
    let area = translate_gdk_rectangle(mon.geometry());
    Monitor::new(
        mon.is_primary(),
        area,
        mon.get_property_workarea()
            .map(translate_gdk_rectangle)
            .unwrap_or(area),
    )
}

pub(crate) fn get_monitors() -> Vec<Monitor> {
    if !gtk::is_initialized() {
        if let Err(err) = gtk::init() {
            tracing::error!("{}", err.message);
            return Vec::new();
        }
    }
    DisplayManager::get()
        .list_displays()
        .iter()
        .flat_map(|display: &Display| {
            (0..display.n_monitors())
                .map(move |i| display.monitor(i).map(translate_gdk_monitor))
                .flatten()
        })
        .collect()
}
