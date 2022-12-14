// SPDX-License-Identifier: MPL-2.0-only
use crate::dock_list::DockList;
use crate::dock_list::DockListType;
use cascade::cascade;
use cosmic_panel_config::{CosmicPanelConfig, PanelAnchor};
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::Orientation;
use gtk4::Separator;
use gtk4::{gio, glib};

mod imp;

glib::wrapper! {
    pub struct AppsContainer(ObjectSubclass<imp::AppsContainer>)
        @extends gtk4::Widget, gtk4::Box,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl AppsContainer {
    pub fn new() -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create AppsContainer");
        let imp = imp::AppsContainer::from_instance(&self_);

        cascade! {
            &self_;
            ..set_orientation(Orientation::Horizontal);
            ..add_css_class("transparent");
            // ..add_css_class("dock_container");
        };

        let saved_app_list_view = DockList::new(DockListType::Saved);
        self_.append(&saved_app_list_view);

        let separator_container = cascade! {
            gtk4::Box::new(Orientation::Vertical, 0);
            ..set_margin_top(8);
            ..set_margin_bottom(8);
            ..set_vexpand(true);
        };
        self_.append(&separator_container);
        let separator = cascade! {
            Separator::new(Orientation::Vertical);
            ..set_margin_start(8);
            ..set_margin_end(8);
            ..set_vexpand(true);
            ..add_css_class("dock_separator");
        };
        separator_container.append(&separator);
        let active_app_list_view = DockList::new(DockListType::Active);
        self_.append(&active_app_list_view);

        imp.saved_list.set(saved_app_list_view).unwrap();
        imp.active_list.set(active_app_list_view).unwrap();
        // Setup
        self_.setup_callbacks();
        let anchor = std::env::var("COSMIC_PANEL_ANCHOR")
            .ok()
            .and_then(|anchor| anchor.parse::<PanelAnchor>().ok())
            .unwrap_or_default();
        self_.set_position(anchor);

        Self::setup_callbacks(&self_);

        self_
    }

    pub fn model(&self, type_: DockListType) -> &gio::ListStore {
        // Get state
        let imp = imp::AppsContainer::from_instance(self);
        match type_ {
            DockListType::Active => imp.active_list.get().unwrap().model(),
            DockListType::Saved => imp.saved_list.get().unwrap().model(),
        }
    }

    pub fn set_position(&self, position: PanelAnchor) {
        self.set_orientation(position.into());
        let imp = imp::AppsContainer::from_instance(self);
        imp.saved_list.get().unwrap().set_position(position);
        imp.active_list.get().unwrap().set_position(position);
    }

    fn setup_callbacks(&self) {
        // Get state
        let imp = imp::AppsContainer::from_instance(self);
        let drop_controller = imp.saved_list.get().unwrap().drop_controller();

        // hack to prevent hiding window when dnd from other apps
        drop_controller.connect_enter(move |_self, _x, _y| gtk4::gdk::DragAction::COPY);
    }
}
